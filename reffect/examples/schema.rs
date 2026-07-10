use reffect::schema::Schema;
use schemars::schema_for;
use std::{fs::File, io::BufWriter, path::PathBuf};

const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let path = PathBuf::from(MANIFEST).join("../docs/schema.json");
    let file = File::create(path).expect("failed to open output file");
    let schema = schema_for!(Schema);
    serde_json::to_writer_pretty(BufWriter::new(file), &schema).expect("failed to output");
}
