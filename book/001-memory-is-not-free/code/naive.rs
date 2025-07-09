use std::collections::HashMap;

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

pub fn parse_logs(input: &str) -> Vec<LogEntry> {
    input
        .lines()
        .filter_map(|line| parse_single_log(line))
        .collect()
}

fn parse_single_log(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() < 3 {
        return None;
    }
    
    let mut metadata = HashMap::new();
    if parts.len() > 3 {
        for item in &parts[3..] {
            if let Some((key, value)) = item.split_once('=') {
                metadata.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    Some(LogEntry {
        timestamp: parts[0].to_string(),
        level: parts[1].to_string(),
        message: parts[2].to_string(),
        metadata,
    })
}
