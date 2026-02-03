use crate::msg_descriptor::{MSG_RULES_SCHEMA, MsgExtractInfo, MsgDescriptor, MsgType};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::sync::OnceLock;

static MSG_DESCRIPTORS: OnceLock<Vec<MsgDescriptor>> = OnceLock::new();
static MSGS_EXTRACT_INFO: OnceLock<Vec<MsgExtractInfo>> = OnceLock::new();

fn validate_against_schema(json_data: &serde_json::Value) -> Result<(), serde_json::Error> {
    let schema: serde_json::Value = serde_json::from_str(MSG_RULES_SCHEMA).map_err(|e| {
        serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "Error:failure when parsing file: '{}'. Schema parsing failed: {}",
                crate::msg_rules_schema_path!(),
                e
            ),
        ))
    })?;
    let compiled_schema = jsonschema::JSONSchema::compile(&schema).map_err(|e| {
        serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Schema compilation failed: {}", e),
        ))
    })?;

    if let Err(errors) = compiled_schema.validate(json_data) {
        let error_messages: Vec<String> = errors.map(|e| e.to_string()).collect();
        return Err(serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("JSON validation failed: {}", error_messages.join(", ")),
        )));
    }

    Ok(())
}

pub fn load_json(path: &str) -> Result<(), serde_json::Error> {
    let file = File::open(path).map_err(|e| serde_json::Error::io(e))?;
    let reader = BufReader::new(file);
    let json_data: serde_json::Value = serde_json::from_reader(reader)?;

    validate_against_schema(&json_data)?;

    // Parse JSON into message descriptors
    let descriptors = parse_msg_descriptors(&json_data)?;
    let _ = MSGS_EXTRACT_INFO.set(descriptors);

    Ok(())
}

fn parse_msg_descriptors(
    json_data: &serde_json::Value,
) -> Result<Vec<MsgExtractInfo>, serde_json::Error> {
    let rules_array = json_data
        .as_array()
        .ok_or_else(|| {
            serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected JSON array at root",
            ))
        })?;

    let mut descriptors = Vec::new();

    for rule in rules_array {
        let regex_start = rule
            .get("regex_start")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Missing or invalid 'regex_start' field",
                ))
            })?
            .to_string();

        let msg_type_str = rule
            .get("msg_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Unspecified");
        let msg_type = MsgType::from_string(msg_type_str);

        let msg_nr_of_lines = rule
            .get("msg_nr_of_lines")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;

        let use_regex_stop = rule
            .get("use_regex_stop")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let regex_stop = rule
            .get("regex_stop")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        descriptors.push(MsgExtractInfo {
            regex_start: regex_start,
            msg_type: msg_type,
            content: "".to_string(),
            msg_nr_of_lines: msg_nr_of_lines,
            use_regex_stop: use_regex_stop,
            regex_stop: regex_stop,
        });
    }

    Ok(descriptors)
}

pub fn get_msg_extract_info() -> Option<&'static Vec<MsgExtractInfo>> {
    MSGS_EXTRACT_INFO.get()
}
