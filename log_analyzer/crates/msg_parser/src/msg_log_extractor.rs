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
                    start_regex_compiled: { start_regex_compiled },
                    stop_regex_compiled: { stop_regex_compiled },
                }
            })
            .collect()
    });

    let f_handle = File::open(in_file)?;
    let reader = BufReader::new(f_handle);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut msgs_with_stop_regex = Vec::new();
    let mut line_idx = 0usize;

    while line_idx < lines.len() {
        if let Some((extraced_msg, consumed_lines)) =
            try_extract_message_with_stop_regex(line_idx, &lines)?
        {
            msgs_with_stop_regex.push(extraced_msg);
            line_idx += consumed_lines;
        }
        line_idx += 1;
    }

    Ok(())
}

fn try_extract_message_with_stop_regex(
    line_idx: usize,
    lines: &[String],
) -> Result<Option<(MsgDescriptor, usize)>, Box<dyn std::error::Error>> {
    if line_idx >= lines.len() {
        return Ok(None);
    }

    let msgs_extract_info = msg_rules_loader::get_msg_extract_info()
        .ok_or("Rules to extract messages not available")?;

    let current_line = &lines[line_idx];

    for msg_extract_info in msgs_extract_info.iter() {
        if !msg_extract_info.use_regex_stop {
            continue;
        }

        let start_regex = Regex::new(&msg_extract_info.regex_start)?;
        let stop_regex = Regex::new(&msg_extract_info.regex_stop)?;

        if !start_regex.is_match(current_line) {
            continue;
        }

        let mut end_idx = line_idx;
        // Check stop on current line and not on the next one to catch corner case where a single line
        // has both start and stop regexes.
        let mut found_stop = stop_regex.is_match(current_line);

        if !found_stop {
            for idx in (line_idx + 1)..lines.len() {
                if stop_regex.is_match(&lines[idx]) {
                    end_idx = idx;
                    found_stop = true;
                    break;
                }
            }
        }

        if !found_stop {
            return Ok(None);
        }

        let content = lines[line_idx..=end_idx].join("\n");
        let new_msg_descriptor = MsgDescriptor {
            extract_info: msg_extract_info.clone(),
            content,
            line_start: line_idx + 1,
            line_end: end_idx + 1,
            file_name: String::new(),
        };

        let consumed_lines = end_idx - line_idx + 1;
        return Ok(Some((new_msg_descriptor, consumed_lines)));
    }

    Ok(None)
}
