use reffect::schema::Schema;
use schemars::schema_for;
use std::{fs::File, io::BufWriter};

const OUT: &str = "schema.json";

fn main() {
    let file = File::create(OUT).expect("failed to open file");
    let schema = schema_for!(Schema);
    serde_json::to_writer_pretty(BufWriter::new(file), &schema).expect("failed to output");
}
