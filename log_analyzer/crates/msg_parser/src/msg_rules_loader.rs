
use crate::msg_descriptor::MsgDescriptor;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;

static JSON_DATA: OnceLock<serde_json::Value> = OnceLock::new();

pub fn load_json(path: &str) -> Result<(), serde_json::Error> {
    let file = File::open(path)
        .map_err(|e| serde_json::Error::io(e))?;
    let reader = BufReader::new(file);
    let json_data: serde_json::Value = serde_json::from_reader(reader)?;

    let _ = JSON_DATA.set(json_data);
    Ok(())
}

pub fn get_json_data() -> Option<&'static serde_json::Value> {
    JSON_DATA.get()
}