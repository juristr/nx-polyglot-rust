use std::{fs, path::PathBuf};

use security_contract::security_snapshot_schema;

fn main() {
    let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let schema_dir = package_root.join("schema");
    let schema_path = schema_dir.join("security-snapshot.schema.json");

    fs::create_dir_all(&schema_dir).expect("create schema directory");
    let json = serde_json::to_string_pretty(&security_snapshot_schema()).expect("serialize schema");
    fs::write(&schema_path, format!("{json}\n")).expect("write schema");

    println!("generated {}", schema_path.display());
}
