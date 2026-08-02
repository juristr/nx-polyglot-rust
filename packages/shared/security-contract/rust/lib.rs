use schemars::{JsonSchema, Schema, schema_for};
use topcoat_security_domain::{SecuritySnapshot, SecurityStreamEvent};

#[allow(dead_code)]
#[derive(JsonSchema)]
struct SecurityContract {
    snapshot: SecuritySnapshot,
    stream_event: SecurityStreamEvent,
}

#[must_use]
pub fn security_snapshot_schema() -> Schema {
    schema_for!(SecurityContract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_contains_snapshot_and_stream_event() {
        let schema = security_snapshot_schema();
        let json = serde_json::to_value(schema).expect("schema serializes");

        assert_eq!(json["title"], "SecurityContract");
        assert!(json["properties"]["snapshot"].is_object());
        assert!(json["properties"]["stream_event"].is_object());
    }
}
