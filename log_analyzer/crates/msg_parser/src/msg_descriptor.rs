#[macro_export]
macro_rules! msg_rules_schema_path {
    () => {
        "msg_rules.schema.json"
    };
}
// Embed the schema file directly into the binary at compile time
// The schema defines the contract for the data structures in this file
pub const MSG_RULES_SCHEMA: &str = include_str!(msg_rules_schema_path!());

pub enum MsgType {
    Info,
    Debug,
    Warning,
    Error,
    Unspecified,
}

impl MsgType {
    pub fn to_string(&self) -> String {
        match self {
            MsgType::Info => "Info".to_string(),
            MsgType::Debug => "Debug".to_string(),
            MsgType::Warning => "Warning".to_string(),
            MsgType::Error => "Error".to_string(),
            MsgType::Unspecified => "Unspecified".to_string(),
        }
    }
    pub fn from_string(s: &str) -> MsgType {
        match s.to_lowercase().as_str() {
            "info" => MsgType::Info,
            "debug" => MsgType::Debug,
            "warning" => MsgType::Warning,
            "error" => MsgType::Error,
            _ => MsgType::Unspecified,
        }
    }
}

pub struct MsgExtractInfo {
    pub regex_start: String,
    pub msg_type: MsgType,
    pub content: String,
    pub msg_nr_of_lines: usize,

    pub use_regex_stop: bool,
    pub regex_stop: String,
}

pub struct MsgDescriptor {
    pub extract_info: MsgExtractInfo,

    /// Message start line in original log file
    pub line_start: usize,
    /// Message end line in original log file
    pub line_end: usize,

    /// Source file name from which the message was read
    pub file_name: String,
}
