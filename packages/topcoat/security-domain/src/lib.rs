use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Active,
    Blocked,
    Investigating,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttackVector {
    CredentialStuffing,
    Ddos,
    Malware,
    Phishing,
    SupplyChain,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct GeoLocation {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct SecurityIncident {
    pub id: String,
    pub sequence: u64,
    pub detected_at: u64,
    pub source: GeoLocation,
    pub target: GeoLocation,
    pub severity: Severity,
    pub status: IncidentStatus,
    pub vector: AttackVector,
    pub blocked_requests: u32,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct SecuritySummary {
    pub active_incidents: usize,
    pub attacks_per_minute: u32,
    pub critical_incidents: usize,
    pub blocked_percent: f64,
    pub top_target: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct SecurityTrendPoint {
    pub label: String,
    pub attacks: u32,
    pub critical: u32,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct VectorBreakdown {
    pub vector: AttackVector,
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct SecuritySnapshot {
    pub sequence: u64,
    pub generated_at: u64,
    pub summary: SecuritySummary,
    pub trend: Vec<SecurityTrendPoint>,
    pub vectors: Vec<VectorBreakdown>,
    pub incidents: Vec<SecurityIncident>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
pub struct SecurityStreamEvent {
    pub incident: SecurityIncident,
    pub snapshot: SecuritySnapshot,
}
