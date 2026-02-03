use crate::msg_rules_loader;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct ParserConfig {
    /// Paths to the log files to be processed
    pub log_file_paths: String, //TODO: change to Vec<String> for multiple files
    pub msg_rules_path: String,
    pub output_path: String,
}

/// Main entry point for processing log files with message rules
///
/// This function coordinates the entire log processing workflow:
/// 1. Load and validate message rules from JSON schema
/// 2. Read and parse the log file
/// 3. Apply message rules to extract structured data
/// 4. Output results
pub fn run(parser_cfg: &ParserConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Load and validate message rules
    msg_rules_loader::load_json(parser_cfg.msg_rules_path.as_str())
        .map_err(|e| format!("Failed to load message rules: {}", e))?;

    // Step 2: Open log file
    let log_file = File::open(parser_cfg.log_file_paths.as_str()).map_err(|e| {
        format!(
            "Failed to open log file '{}': {}",
            parser_cfg.log_file_paths.as_str(),
            e
        )
    })?;

    let reader = BufReader::new(log_file);

    // Step 3: Process log file line by line
    let mut processed_count = 0;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;

        // TODO: Apply message rules to parse each line
        // This is where you would match against msg_descriptor patterns

        processed_count += 1;
    }

    Ok(())
}
