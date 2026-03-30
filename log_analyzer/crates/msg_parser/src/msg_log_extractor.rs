use crate::msg_descriptor::{MsgDescriptor, MsgExtractInfo};
use crate::msg_rules_loader;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;

struct MsgExtractInfoWithStopRegex {
    msg_extract_info: MsgExtractInfo,
    start_regex_compiled: Regex,
    stop_regex_compiled: Regex,
}

static MSG_RULES_LOADER_EXTRACT_INFO_WITH_STOP_REGEX: OnceLock<Vec<MsgExtractInfoWithStopRegex>> =
    OnceLock::new();

static MSG_DESCRIPTORS: OnceLock<Vec<MsgDescriptor>> = OnceLock::new();

/// Extracts messages from a log file based on loaded extraction rules.
///
/// This function reads the log file line by line and matches against
/// regex patterns to identify message boundaries.
///
/// # Arguments
/// * `in_file` - Path to the input log file
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, error otherwise
pub fn extract_msgs_from_file(in_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    extract_multi_line_msg_with_start_stop_regex(in_file)?;

    Ok(())
}

fn extract_multi_line_msg_with_start_stop_regex(
    in_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    // compile regexes for all MsgExtractInfo with stop regex
    let _ = MSG_RULES_LOADER_EXTRACT_INFO_WITH_STOP_REGEX.get_or_init(|| {
        let msgs_extract_info = msg_rules_loader::get_msg_extract_info()
            .expect("Rules to extract messages not available");

        msgs_extract_info
            .iter()
            .map(|msg_extract_info| {
                let start_regex_compiled = Regex::new(&msg_extract_info.regex_start).unwrap();
                let stop_regex_compiled = Regex::new(&msg_extract_info.regex_stop).unwrap();
                MsgExtractInfoWithStopRegex {
                    msg_extract_info: msg_extract_info.clone(),
                    start_regex_compiled: {start_regex_compiled},
                    stop_regex_compiled: {stop_regex_compiled},
                }
            })
            .collect()
    });


    let f_handle = File::open(in_file)?;
    let reader = BufReader::new(f_handle);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut msgs_with_stop_regex = Vec::new();
    let mut line_idx = 0usize;

    let mut start_regex_found = false;
    while line_idx < lines.len() {
        line_idx += 1; // TODO: remove line. Currently needed to avoid infinite loop.
        if let Some((extraced_msg, consumed_lines)) = try_extract_message_with_stop_regex(line_idx, &lines)? {
            msgs_with_stop_regex.push(extraced_msg);
        } else {
            line_idx += 1;
        }
    }

    Ok(())
}

fn try_extract_message_with_stop_regex(
    line_idx: usize,
    lines: &[String],
) -> Result<Option<(MsgDescriptor, usize)>, Box<dyn std::error::Error>> {
    let msgs_extract_info = msg_rules_loader::get_msg_extract_info()
        .ok_or("Rules to extract messages not available")?;

    let mut new_msg_descriptor: MsgDescriptor = MsgDescriptor::default();
    let mut consumed_lines = 0usize;
    Ok(Some((new_msg_descriptor, consumed_lines)))
}
