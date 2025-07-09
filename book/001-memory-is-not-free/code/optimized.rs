use std::collections::HashMap;

#[derive(Debug)]
pub struct LogEntry<'a> {
    pub timestamp: &'a str,
    pub level: &'a str,
    pub message: &'a str,
    pub metadata: Vec<(&'a str, &'a str)>,
}

pub fn parse_logs(input: &str) -> Vec<LogEntry> {
    let mut entries = Vec::with_capacity(input.lines().count());
    
    for line in input.lines() {
        if let Some(entry) = parse_single_log_fast(line) {
            entries.push(entry);
        }
    }
    
    entries
}

fn parse_single_log_fast(line: &str) -> Option<LogEntry> {
    // Find delimiters without allocation
    let mut splits = line.match_indices('|');
    
    // Get first split position
    let (first_pos, _) = splits.next()?;
    let timestamp = &line[..first_pos];
    
    // Get second split position
    let (second_pos, _) = splits.next()?;
    let level = &line[first_pos + 1..second_pos];
    
    // Get message (from after second split to next split or end)
    let message = if let Some((next_pos, _)) = splits.next() {
        &line[second_pos + 1..next_pos]
    } else {
        &line[second_pos + 1..]
    };
    
    // Only parse metadata if needed
    let metadata = if let Some((pos, _)) = splits.next() {
        let metadata_str = &line[pos + 1..];
        metadata_str
            .split('|')
            .filter_map(|pair| pair.split_once('='))
            .collect()
    } else {
        Vec::new()
    };
    
    Some(LogEntry {
        timestamp,
        level,
        message,
        metadata,
    })
}
