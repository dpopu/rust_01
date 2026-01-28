use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

pub struct Config {
    _log_file: String,
    _msg_rules_file: String,
    _out_msgs_file: String,
}

static CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

impl Config {
    #[allow(dead_code)] // Function is present for learning purposes
    pub fn new(log_file: String, msg_rules_file: String, out_msgs_file: String) -> Self {
        Config {
            _log_file: log_file,
            _msg_rules_file: msg_rules_file,
            _out_msgs_file: out_msgs_file,
        }
    }

    #[allow(dead_code)] // Function is present for learning purposes
    pub fn init(log_file: String, msg_rules_file: String, out_msgs_file: String) {
        let config = Config::new(log_file, msg_rules_file, out_msgs_file);
        // Attempt to set the singleton. The OnceLock ensures only the first call succeeds.
        // We ignore the Result to handle cases where init is called multiple times (idempotent).
        let _ = CONFIG.set(Mutex::new(config));
    }

    pub fn get() -> &'static Mutex<Config> {
        // get_or_init() checks if the OnceLock has already been initialized.
        // If yes, it returns the reference to the stored Mutex<Config>.
        // If no, it executes the closure (the code between "||" and "}") to initialize it.

        // The "||" is a closure (similar to lambda in Python).
        // Syntax: |parameters| { body }
        // Here: || means a closure with NO parameters.
        // The closure returns: Mutex::new(Config { ... }) with default empty strings.
        CONFIG.get_or_init(|| {
            Mutex::new(Config {
                _log_file: String::new(),
                _msg_rules_file: String::new(),
                _out_msgs_file: String::new(),
            })
        })
    }

    pub fn set_log_file(value: String) {
        if let Ok(mut config) = Self::get().lock() {
            config._log_file = value;
        }
    }

    pub fn get_log_file() -> String {
        Self::get()
            .lock()
            .map(|config| config._log_file.clone())
            .unwrap_or_default()
    }

    pub fn set_msg_rules_file(value: String) {
        if let Ok(mut config) = Self::get().lock() {
            config._msg_rules_file = value;
        }
    }

    pub fn get_msg_rules_file() -> String {
        Self::get()
            .lock()
            .map(|config| config._msg_rules_file.clone())
            .unwrap_or_default()
    }

    pub fn set_out_msgs_file(value: String) {
        if let Ok(mut config) = Self::get().lock() {
            config._out_msgs_file = value;
        }
    }

    pub fn get_out_msgs_file() -> String {
        Self::get()
            .lock()
            .map(|config| config._out_msgs_file.clone())
            .unwrap_or_default()
    }

    /// Checks if the given string is a valid Windows path.
    /// A string is valid if it does not contain any of the following characters:
    /// `<`, `>`, `"`, `/`, `\\`, `|`, `?`, or `*`.
    /// TODO: move the function to a more generic library.
    pub fn is_valid_windows_path(s: &str) -> bool {
        const INVALID: &[char] = &['<', '>', '"', '|', '?', '*'];
        !s.chars().any(|c| INVALID.contains(&c))
    }
    pub fn validate() -> () {
        let config = Self::get().lock().unwrap();

        let path = Path::new(config._log_file.as_str());
        if !path.is_file() {
            eprintln!(
                "Log file does not exist or is not a file: {}",
                config._log_file
            );
            std::process::exit(1);
        }

        let path = Path::new(config._msg_rules_file.as_str());
        if !path.is_file() {
            eprintln!(
                "Message rules file does not exist or is not a file: {}",
                config._msg_rules_file
            );
            std::process::exit(1);
        }

        let path = Path::new(config._out_msgs_file.as_str());
        if !Config::is_valid_windows_path(path.to_str().unwrap_or("")) {
            eprintln!(
                "Output messages file is not a file path: {}",
                config._out_msgs_file
            );
            std::process::exit(1);
        }
    }
}
