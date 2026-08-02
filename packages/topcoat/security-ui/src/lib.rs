use topcoat::{
    Result,
    view::{component, view},
};
use topcoat_security_domain::{AttackVector, SecuritySnapshot, Severity};

const STYLES: &str = r#"
:root { color-scheme: dark; font-family: Inter, ui-sans-serif, system-ui, sans-serif; background: #05080d; color: #eef5ff; }
* { box-sizing: border-box; }
body { margin: 0; min-width: 320px; background: #05080d; }
.soc-shell { min-height: 100vh; display: grid; grid-template-columns: 248px minmax(0, 1fr); }
.soc-sidebar { padding: 28px 20px; border-right: 1px solid #1c2634; background: #080d14; display: flex; flex-direction: column; }
.soc-brand { display: flex; align-items: center; gap: 11px; font-weight: 760; letter-spacing: -.02em; }
.soc-mark { width: 34px; height: 34px; display: grid; place-items: center; border: 1px solid #344359; border-radius: 8px; color: #8df5d0; font: 700 10px/1 ui-monospace, monospace; }
.soc-classification { margin: 42px 8px 12px; color: #66758a; font: 700 9px/1 ui-monospace, monospace; letter-spacing: .14em; text-transform: uppercase; }
.soc-nav { display: grid; gap: 4px; }
.soc-nav a { padding: 11px 12px; border-radius: 7px; color: #78889e; text-decoration: none; font-size: 12px; }
.soc-nav a:first-child { color: #dce9f7; background: #131c28; }
.soc-system { margin-top: auto; padding: 14px; border: 1px solid #1c2938; border-radius: 9px; color: #7e8da1; font: 10px/1.7 ui-monospace, monospace; }
.soc-system strong { display: block; color: #85e3c3; font-size: 10px; }
.soc-live-dot { width: 7px; height: 7px; display: inline-block; margin-right: 7px; border-radius: 50%; background: #54e8b4; box-shadow: 0 0 12px #54e8b4; }
.soc-main { min-width: 0; padding: 32px 36px 44px; }
.soc-header { display: flex; justify-content: space-between; gap: 24px; align-items: flex-start; margin-bottom: 22px; }
.soc-eyebrow, .soc-kicker { margin: 0; color: #718198; font: 700 9px/1 ui-monospace, monospace; letter-spacing: .14em; text-transform: uppercase; }
.soc-header h1 { margin: 7px 0 6px; font-size: clamp(27px, 3vw, 40px); letter-spacing: -.045em; }
.soc-header p:last-child { margin: 0; color: #7e8da0; font-size: 12px; }
.soc-status { padding: 10px 13px; border: 1px solid #1e3d35; border-radius: 8px; color: #7ee8c2; background: #0b1918; font: 700 10px/1 ui-monospace, monospace; }
.soc-metrics { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 11px; margin-bottom: 11px; }
.soc-card, .soc-panel { border: 1px solid #1b2634; border-radius: 10px; background: #0a1018; }
.soc-card { padding: 16px 17px; }
.soc-card span { color: #708096; font-size: 10px; }
.soc-card strong { display: block; margin: 10px 0 4px; font: 690 25px/1 ui-monospace, monospace; letter-spacing: -.05em; }
.soc-card small { color: #516175; font-size: 9px; }
.soc-card.is-alert strong { color: #ff7d73; }
.soc-card.is-safe strong { color: #78e7be; }
.soc-grid { display: grid; grid-template-columns: minmax(0, 1.65fr) minmax(320px, .8fr); gap: 11px; margin-bottom: 11px; }
.soc-panel { min-width: 0; padding: 17px; }
.soc-panel-head { display: flex; justify-content: space-between; align-items: flex-start; gap: 16px; margin-bottom: 13px; }
.soc-panel h2 { margin: 5px 0 0; font-size: 14px; letter-spacing: -.015em; }
.soc-panel-code { color: #4d6076; font: 9px/1 ui-monospace, monospace; }
.soc-globe-frame { position: relative; height: 480px; overflow: hidden; border: 1px solid #162333; border-radius: 8px; background: #03070b; }
security-globe { display: block; width: 100%; height: 100%; }
.soc-globe-overlay { position: absolute; left: 14px; bottom: 14px; display: grid; gap: 5px; padding: 10px 12px; border: 1px solid #213043; border-radius: 7px; color: #6d7e93; background: rgb(5 10 16 / 88%); font: 9px/1.4 ui-monospace, monospace; pointer-events: none; }
.soc-globe-overlay strong { color: #d9e8f7; font-size: 11px; }
.soc-feed { display: grid; gap: 7px; }
.soc-incident { padding: 11px; border: 1px solid #182433; border-radius: 7px; background: #090f16; }
.soc-incident-top { display: flex; justify-content: space-between; gap: 10px; align-items: center; }
.soc-incident code { color: #8ea0b6; font-size: 9px; }
.soc-severity { padding: 4px 6px; border-radius: 4px; font: 700 8px/1 ui-monospace, monospace; text-transform: uppercase; }
.soc-severity.is-critical { color: #ff8b82; background: #2a1215; }
.soc-severity.is-high { color: #f4b477; background: #251b11; }
.soc-severity.is-medium { color: #77bdf3; background: #101f2d; }
.soc-severity.is-low { color: #8ba0b6; background: #151c25; }
.soc-route { margin: 9px 0 5px; color: #d8e3ee; font-size: 10px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.soc-vector { display: flex; justify-content: space-between; color: #57687d; font: 8px/1.3 ui-monospace, monospace; }
.soc-bottom { display: grid; grid-template-columns: 1.45fr 1fr; gap: 11px; }
.soc-table { overflow-x: auto; }
.soc-row { min-width: 640px; display: grid; grid-template-columns: .8fr 1.25fr .9fr .7fr .8fr; gap: 12px; align-items: center; min-height: 40px; border-top: 1px solid #16212e; color: #8b9bae; font-size: 9px; }
.soc-row:first-child { min-height: 28px; border-top: 0; color: #4f6074; font: 700 8px/1 ui-monospace, monospace; text-transform: uppercase; }
.soc-row code { color: #b4c3d3; }
.soc-runtime { display: grid; gap: 11px; }
.soc-runtime-row { display: flex; justify-content: space-between; padding-bottom: 10px; border-bottom: 1px solid #182330; color: #74859a; font-size: 9px; }
.soc-runtime-row strong { color: #dce8f5; font: 600 10px/1 ui-monospace, monospace; }
@media (max-width: 1100px) { .soc-shell { grid-template-columns: 1fr; } .soc-sidebar { display: none; } .soc-grid, .soc-bottom { grid-template-columns: 1fr; } }
@media (max-width: 700px) { .soc-main { padding: 24px 14px 32px; } .soc-metrics { grid-template-columns: repeat(2, 1fr); } .soc-header { display: block; } .soc-status { margin-top: 14px; width: fit-content; } .soc-globe-frame { height: 390px; } }
"#;

#[must_use]
pub fn vector_label(vector: AttackVector) -> &'static str {
    match vector {
        AttackVector::CredentialStuffing => "Credential stuffing",
        AttackVector::Ddos => "DDoS",
        AttackVector::Malware => "Malware",
        AttackVector::Phishing => "Phishing",
        AttackVector::SupplyChain => "Supply chain",
    }
}

fn severity_class(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "critical",
        Severity::High => "high",
        Severity::Medium => "medium",
        Severity::Low => "low",
    }
}

#[component]
pub async fn security_dashboard(snapshot: &SecuritySnapshot) -> Result {
    let blocked = format!("{:.1}%", snapshot.summary.blocked_percent);
    let attacks = snapshot.summary.attacks_per_minute.to_string();
    let active = snapshot.summary.active_incidents.to_string();
    let critical = snapshot.summary.critical_incidents.to_string();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="description" content="Topcoat security operations showcase">
                <title>"Sentinel // Global Threat Operations"</title>
                <style>(STYLES)</style>
                topcoat::dev::script()
                <script type="module" src="/assets/security-globe.js"></script>
            </head>
            <body>
                <div class="soc-shell">
                    <aside class="soc-sidebar">
                        <div class="soc-brand"><span class="soc-mark">"S//C"</span><span>"Sentinel Command"</span></div>
                        <p class="soc-classification">"Threat operations"</p>
                        <nav class="soc-nav">
                            <a href="#overview">"Global overview"</a>
                            <a href="#incidents">"Incident queue"</a>
                            <a href="#systems">"Protected systems"</a>
                            <a href="/api/security/snapshot">"JSON API"</a>
                        </nav>
                        <div class="soc-system">
                            <strong><span class="soc-live-dot"></span>"STREAM ONLINE"</strong>
                            "TOPCOAT 0.5 / RUST 1.95"<br>
                            "NX PROJECT: topcoat-security"
                        </div>
                    </aside>
                    <main class="soc-main" id="overview">
                        <header class="soc-header">
                            <div>
                                <p class="soc-eyebrow">"Realtime perimeter intelligence"</p>
                                <h1>"Global threat operations"</h1>
                                <p>"Deterministic in-memory incidents generated continuously by the Rust server."</p>
                            </div>
                            <div class="soc-status"><span class="soc-live-dot"></span>"LIVE // SSE"</div>
                        </header>
                        <section class="soc-metrics" aria-label="Security summary">
                            <article class="soc-card is-alert"><span>"Active incidents"</span><strong>(active)</strong><small>"Requires analyst attention"</small></article>
                            <article class="soc-card"><span>"Attacks / minute"</span><strong>(attacks)</strong><small>"Current generated rate"</small></article>
                            <article class="soc-card is-alert"><span>"Critical severity"</span><strong>(critical)</strong><small>"Across retained incidents"</small></article>
                            <article class="soc-card is-safe"><span>"Traffic blocked"</span><strong>(blocked)</strong><small>"Automated containment"</small></article>
                        </section>
                        <section class="soc-grid">
                            <article class="soc-panel">
                                <div class="soc-panel-head"><div><p class="soc-kicker">"WebGL telemetry"</p><h2>"Attack vectors in motion"</h2></div><span class="soc-panel-code">"Globe.GL / TypeScript package"</span></div>
                                <div class="soc-globe-frame">
                                    <security-globe></security-globe>
                                    <div class="soc-globe-overlay"><span>"HIGHEST TARGET DENSITY"</span><strong>(&snapshot.summary.top_target)</strong><span>"EVENT " (&snapshot.sequence)</span></div>
                                </div>
                            </article>
                            <article class="soc-panel">
                                <div class="soc-panel-head"><div><p class="soc-kicker">"Incoming feed"</p><h2>"Latest detections"</h2></div><span class="soc-panel-code">"SERVER-SENT EVENTS"</span></div>
                                <div class="soc-feed">
                                    for incident in snapshot.incidents.iter().take(7) {
                                        <article class="soc-incident">
                                            <div class="soc-incident-top"><code>(&incident.id)</code><span class=(format!("soc-severity is-{}", severity_class(incident.severity)))>(severity_class(incident.severity))</span></div>
                                            <p class="soc-route">(&incident.source.city) " → " (&incident.target.city)</p>
                                            <div class="soc-vector"><span>(vector_label(incident.vector))</span><span>(incident.blocked_requests) " blocked"</span></div>
                                        </article>
                                    }
                                </div>
                            </article>
                        </section>
                        <section class="soc-bottom" id="incidents">
                            <article class="soc-panel soc-table">
                                <div class="soc-panel-head"><div><p class="soc-kicker">"In-memory queue"</p><h2>"Incident tracker"</h2></div></div>
                                <div class="soc-row"><span>"Incident"</span><span>"Route"</span><span>"Vector"</span><span>"Severity"</span><span>"Status"</span></div>
                                for incident in snapshot.incidents.iter().take(8) {
                                    <div class="soc-row"><code>(&incident.id)</code><span>(&incident.source.city) " → " (&incident.target.city)</span><span>(vector_label(incident.vector))</span><span>(severity_class(incident.severity))</span><span>(format!("{:?}", incident.status))</span></div>
                                }
                            </article>
                            <article class="soc-panel" id="systems">
                                <div class="soc-panel-head"><div><p class="soc-kicker">"Nx orchestration"</p><h2>"Polyglot runtime graph"</h2></div></div>
                                <div class="soc-runtime">
                                    <div class="soc-runtime-row"><span>"Incident API + SSE"</span><strong>"Rust / Topcoat"</strong></div>
                                    <div class="soc-runtime-row"><span>"Domain + queue"</span><strong>"Rust packages"</strong></div>
                                    <div class="soc-runtime-row"><span>"Interactive globe"</span><strong>"TypeScript / Globe.GL"</strong></div>
                                    <div class="soc-runtime-row"><span>"Task orchestration"</span><strong>"Nx / Monodon"</strong></div>
                                </div>
                            </article>
                        </section>
                    </main>
                </div>
            </body>
        </html>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_attack_vectors() {
        assert_eq!(
            vector_label(AttackVector::CredentialStuffing),
            "Credential stuffing"
        );
        assert_eq!(vector_label(AttackVector::Ddos), "DDoS");
    }
}
