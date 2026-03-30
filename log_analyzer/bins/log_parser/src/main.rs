mod config;
mod config_loader;

use config::*;
use msg_parser::processor;

use config_loader::cli_parser;

fn main() {
    cli_parser();
    Config::validate();

    let parser_cfg = processor::ParserConfig {
        log_file_path: Config::get_log_file(),
        msg_rules_path: Config::get_msg_rules_file(),
        output_path: Config::get_out_msgs_file(),
    };
    // Run the log processor with configuration values
    match processor::run(&parser_cfg) {
        Ok(()) => println!("Log processing completed successfully"),
        Err(e) => {
            eprintln!("ERROR: Log processing failed: {}", e);
            std::process::exit(1);
        }
    }
}
