pub enum MsgType{
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

    pub struct MsgDescriptor {
        pub regex_start: String,
        pub msg_type: MsgType,
        pub content: String,
        pub msg_nr_of_lines: usize,

        pub use_regex_stop: bool,
        pub regex_stop: String,
    }