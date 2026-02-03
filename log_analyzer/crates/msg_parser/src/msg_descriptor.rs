use std::str::FromStr;

#[macro_export]
macro_rules! msg_rules_schema_path {
    () => {
        "msg_rules.schema.json"
    };
}
// Embed the schema file directly into the binary at compile time
// The schema defines the contract for the data structures in this file
pub const MSG_RULES_SCHEMA: &str = include_str!(msg_rules_schema_path!());

/// Represents the type and severity level of a log message.
///
/// This enum categorizes log messages into different severity levels
/// for filtering and processing purposes.
pub enum MsgType {
    Info,
    Debug,
    Warning,
    Error,
    /// Fallback variant for messages that don't fit any other category.
    ///
    /// **Note:** This should only be used when none of the other message types apply.
    /// Users should try to avoid this value as much as possible and prefer using
    /// a more specific type instead.
    Unspecified,
}

impl MsgType {
    pub fn to_str(&self) -> &str {
        match self {
            MsgType::Info => "Info",
            MsgType::Debug => "Debug",
            MsgType::Warning => "Warning",
            MsgType::Error => "Error",
            MsgType::Unspecified => "Unspecified",
        }
    }
    // pub fn from_string(s: &str) -> MsgType {
    //     match s.to_lowercase().as_str() {
    //         "info" => MsgType::Info,
    //         "debug" => MsgType::Debug,
    //         "warning" => MsgType::Warning,
    //         "error" => MsgType::Error,
    //         _ => MsgType::Unspecified,
    //     }
    // }
}

impl FromStr for MsgType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" => Ok(MsgType::Info),
            "debug" => Ok(MsgType::Debug),
            "warning" => Ok(MsgType::Warning),
            "error" => Ok(MsgType::Error),
            "unspecifed" => Ok(MsgType::Unspecified),
            _ => Err(format!("Invalid MsgType string: {}", s)),
        }
    }
}

/// Information for extracting messages from log files.
///
/// This struct contains the regex patterns and configuration needed to identify
/// and extract messages based on their start/stop patterns.
pub struct MsgExtractInfo {
    pub regex_start: String,
    /// The type/severity level of the message (Info, Debug, Warning, Error, etc.)
    pub msg_type: MsgType,
    pub msg_nr_of_lines: usize,

    pub use_regex_stop: bool,
    pub regex_stop: String,
}

/// Descriptor for a parsed message from a log file.
///
/// Contains the extracted message content along with metadata about its location
/// in the original log file and the extraction rules used.
pub struct MsgDescriptor {
    pub extract_info: MsgExtractInfo,

    pub content: String,

    /// Message start line in original log file
    pub line_start: usize,
    /// Message end line in original log file
    pub line_end: usize,

    /// Source file name from which the message was read
    pub file_name: String,
}
