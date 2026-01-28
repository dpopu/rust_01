use std::env;

use crate::config::Config;

// Note: This function does not need to return a tuple or anything because we work with
// a singleton Config structure. Configuration values are stored directly in the CONFIG singleton
// and can be accessed from anywhere using Config::get_log_file(), Config::get_msg_rules_file(), etc.
pub fn cli_parser() -> () {
    let args: Vec<String> = env::args().collect();

    let mut log_file: String = String::new();
    let mut msg_rules: String = String::new();
    let mut out_msgs: String = String::new();

    // Parse command-line arguments
    for arg in &args[1..] {
        if arg.starts_with(&"--in_log_file=") {
            log_file = arg.strip_prefix(&"--in_log_file=").unwrap().to_string();
        } else if arg.starts_with(&"--in_msg_rules=") {
            msg_rules = arg.strip_prefix(&"--in_msg_rules=").unwrap().to_string();
        } else if arg.starts_with(&"--out_msgs=") {
            out_msgs = arg.strip_prefix(&"--out_msgs=").unwrap().to_string();
        }
    }

    // Validate parameters
    if !log_file.is_empty() {
        Config::set_log_file(log_file);
    } else {
        eprintln!("Error: --in_log_file parameter is required");
        std::process::exit(1);
    }

    if !msg_rules.is_empty() {
        Config::set_msg_rules_file(msg_rules);
    } else {
        eprintln!("Error: --in_msg_rules parameter is required");
        std::process::exit(1);
    }

    if !out_msgs.is_empty() {
        Config::set_out_msgs_file(out_msgs);
    } else {
        eprintln!("Error: --out_msgs parameter is required");
        std::process::exit(1);
    }
}
