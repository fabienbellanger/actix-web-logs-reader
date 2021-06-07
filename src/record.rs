use chrono::{DateTime, SecondsFormat, Utc};
use colored::Colorize;
use serde::Deserialize;
use colored::ColoredString;

pub const LEVELS: [&str; 6] = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL"];

#[derive(Deserialize, Debug)]
pub struct LogRecord {
    pub time: DateTime<Utc>,
    pub level: String,
    pub msg: String,
}

impl LogRecord {
    /// Format a record
    pub fn format(&self) -> String {
        format!(
            "{} {:>5}: {}\n",
            self.time.to_rfc3339_opts(SecondsFormat::Secs, true).bright_black(),
            format_level(&self.level),
            format_msg(&self.msg),
        )
    }

    /// Display a record if record level is greater or equal to targer level
    pub fn display_record(&self, level: &str) -> bool {
        let target_level_index = LEVELS.iter().position(|&r| r == level).unwrap_or(0);
        let level_index = LEVELS.iter().position(|&r| r == self.level).unwrap_or(0);

        level_index >= target_level_index
    }
}

fn format_level(level: &str) -> ColoredString {
    match LEVELS.iter().position(|&r| r == level).unwrap_or(0) {
        0 => level.to_string().purple().bold(),
        1 => level.to_string().blue().bold(),
        2 => level.to_string().green().bold(),
        3 => level.to_string().yellow().bold(),
        _ => level.to_string().red().bold(),
    }
}

fn format_msg(msg: &str) -> String {
    msg.to_string()
}
