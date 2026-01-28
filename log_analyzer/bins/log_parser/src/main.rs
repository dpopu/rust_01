mod config;
mod config_loader;

use msg_parser::msg_rules_loader::load_json;
use config::*;

use config_loader::cli_parser;

fn main() {
    cli_parser();
    Config::validate();

    let msg_rules_path = Config::get_msg_rules_file();
    match load_json(msg_rules_path.as_str()) {
        Ok(json) => println!("Msg rules loaded successfully: {:?}", json),
        Err(e) => eprintln!("Failed to load msg rules from {}: {}", msg_rules_path, e),
    }

    println!(
        "Content of config after CLI parsing: \n{}\n{}\n{}",
        config::Config::get_log_file(),
        config::Config::get_msg_rules_file(),
        config::Config::get_out_msgs_file()
    );
}
