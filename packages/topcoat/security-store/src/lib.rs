use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::sync::broadcast;
use topcoat_security_domain::{
    AttackVector, GeoLocation, IncidentStatus, SecurityIncident, SecuritySnapshot,
    SecurityStreamEvent, SecuritySummary, SecurityTrendPoint, Severity, VectorBreakdown,
};

const MAX_INCIDENTS: usize = 64;
const MAX_TREND_POINTS: usize = 18;

const LOCATIONS: [(&str, &str, f64, f64); 12] = [
    ("Singapore", "Singapore", 1.3521, 103.8198),
    ("Frankfurt", "Germany", 50.1109, 8.6821),
    ("Ashburn", "United States", 39.0438, -77.4874),
    ("Sao Paulo", "Brazil", -23.5558, -46.6396),
    ("Tokyo", "Japan", 35.6762, 139.6503),
    ("Sydney", "Australia", -33.8688, 151.2093),
    ("London", "United Kingdom", 51.5072, -0.1276),
    ("Mumbai", "India", 19.076, 72.8777),
    ("Johannesburg", "South Africa", -26.2041, 28.0473),
    ("Toronto", "Canada", 43.6532, -79.3832),
    ("Seoul", "South Korea", 37.5665, 126.978),
    ("Dubai", "United Arab Emirates", 25.2048, 55.2708),
];

#[derive(Clone)]
pub struct SecurityRuntime {
    inner: Arc<RwLock<SecurityStore>>,
    events: broadcast::Sender<SecurityStreamEvent>,
}

struct SecurityStore {
    sequence: u64,
    incidents: VecDeque<SecurityIncident>,
    trend: VecDeque<SecurityTrendPoint>,
}

impl Default for SecurityRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityRuntime {
    #[must_use]
    pub fn new() -> Self {
        let (events, _) = broadcast::channel(128);
        let runtime = Self {
            inner: Arc::new(RwLock::new(SecurityStore {
                sequence: 0,
                incidents: VecDeque::new(),
                trend: VecDeque::new(),
            })),
            events,
        };

        for _ in 0..20 {
            runtime.tick();
        }

        runtime
    }

    pub fn tick(&self) -> SecurityStreamEvent {
        let event = {
            let mut store = self.inner.write().expect("security store poisoned");
            store.sequence += 1;
            let incident = incident_for(store.sequence);
            store.incidents.push_front(incident.clone());
            store.incidents.truncate(MAX_INCIDENTS);

            let attacks = 36 + ((store.sequence * 17) % 78) as u32;
            let critical = 2 + ((store.sequence * 3) % 9) as u32;
            let trend = SecurityTrendPoint {
                label: format!("T-{}", MAX_TREND_POINTS.saturating_sub(1)),
                attacks,
                critical,
            };
            store.trend.push_back(trend);
            while store.trend.len() > MAX_TREND_POINTS {
                store.trend.pop_front();
            }

            let snapshot = snapshot_from(&store);
            SecurityStreamEvent { incident, snapshot }
        };

        let _ = self.events.send(event.clone());
        event
    }

    #[must_use]
    pub fn snapshot(&self) -> SecuritySnapshot {
        let store = self.inner.read().expect("security store poisoned");
        snapshot_from(&store)
    }

    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<SecurityStreamEvent> {
        self.events.subscribe()
    }
}

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before epoch")
        .as_millis() as u64
}

fn location(index: usize) -> GeoLocation {
    let (city, country, latitude, longitude) = LOCATIONS[index % LOCATIONS.len()];
    GeoLocation {
        city: city.to_owned(),
        country: country.to_owned(),
        latitude,
        longitude,
    }
}

fn incident_for(sequence: u64) -> SecurityIncident {
    let source_index = (sequence as usize * 5 + 3) % LOCATIONS.len();
    let mut target_index = (sequence as usize * 7 + 1) % LOCATIONS.len();
    if source_index == target_index {
        target_index = (target_index + 1) % LOCATIONS.len();
    }

    let severity = match sequence % 10 {
        0 | 1 => Severity::Critical,
        2..=4 => Severity::High,
        5..=7 => Severity::Medium,
        _ => Severity::Low,
    };
    let status = match sequence % 7 {
        0 => IncidentStatus::Active,
        1 => IncidentStatus::Investigating,
        _ => IncidentStatus::Blocked,
    };
    let vector = match sequence % 5 {
        0 => AttackVector::CredentialStuffing,
        1 => AttackVector::Ddos,
        2 => AttackVector::Malware,
        3 => AttackVector::Phishing,
        _ => AttackVector::SupplyChain,
    };

    SecurityIncident {
        id: format!("INC-{sequence:05}"),
        sequence,
        detected_at: now_millis(),
        source: location(source_index),
        target: location(target_index),
        severity,
        status,
        vector,
        blocked_requests: 180 + ((sequence * 977) % 18_000) as u32,
    }
}

fn snapshot_from(store: &SecurityStore) -> SecuritySnapshot {
    let incidents: Vec<_> = store.incidents.iter().cloned().collect();
    let active_incidents = incidents
        .iter()
        .filter(|incident| incident.status != IncidentStatus::Blocked)
        .count();
    let critical_incidents = incidents
        .iter()
        .filter(|incident| incident.severity == Severity::Critical)
        .count();
    let blocked = incidents
        .iter()
        .filter(|incident| incident.status == IncidentStatus::Blocked)
        .count();
    let blocked_percent = if incidents.is_empty() {
        0.0
    } else {
        blocked as f64 / incidents.len() as f64 * 100.0
    };

    let mut targets = BTreeMap::<String, usize>::new();
    let mut vectors = BTreeMap::<String, (AttackVector, u32)>::new();
    for incident in &incidents {
        *targets.entry(incident.target.city.clone()).or_default() += 1;
        let key = format!("{:?}", incident.vector);
        let entry = vectors.entry(key).or_insert((incident.vector, 0));
        entry.1 += 1;
    }

    let top_target = targets
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map_or_else(|| "None".to_owned(), |(city, _)| city);
    let attacks_per_minute = store
        .trend
        .back()
        .map_or(0, |point| point.attacks.saturating_mul(2));

    SecuritySnapshot {
        sequence: store.sequence,
        generated_at: now_millis(),
        summary: SecuritySummary {
            active_incidents,
            attacks_per_minute,
            critical_incidents,
            blocked_percent,
            top_target,
        },
        trend: store.trend.iter().cloned().collect(),
        vectors: vectors
            .into_values()
            .map(|(vector, count)| VectorBreakdown { vector, count })
            .collect(),
        incidents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_deterministic_sequences() {
        let runtime = SecurityRuntime::new();
        let before = runtime.snapshot();
        let event = runtime.tick();

        assert_eq!(event.incident.sequence, before.sequence + 1);
        assert_eq!(event.snapshot.incidents[0].id, event.incident.id);
        assert!(event.snapshot.summary.blocked_percent > 50.0);
        assert_eq!(event.snapshot.trend.len(), MAX_TREND_POINTS);
    }
}
