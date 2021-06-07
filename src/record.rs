use chrono::{DateTime, SecondsFormat, Utc};
use colored::Colorize;
use serde::Deserialize;
use regex::Regex;

pub const LEVELS: [&str; 6] = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL"];

// TODO: Replace String to &str?
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
            "{} {}: {}\n",
            self.time.to_rfc3339_opts(SecondsFormat::Secs, true).bright_black(),
            self.format_level(),
            self.format_msg(),
        )
    }

    /// Display a record if record level is greater or equal to targer level
    pub fn display_record(&self, level: &str) -> bool {
        let target_level_index = LEVELS.iter().position(|&r| r == level).unwrap_or(0);
        let level_index = LEVELS.iter().position(|&r| r == self.level).unwrap_or(0);

        level_index >= target_level_index
    }

    /// Format record level
    fn format_level(&self) -> String {
        match LEVELS.iter().position(|&r| r == self.level).unwrap_or(0) {
            0 => "TRACE".magenta(),
            1 => "DEBUG".blue(),
            2 => " INFO".green(),
            3 => " WARN".yellow(),
            4 => "ERROR".red(),
            _ => "FATAL".red(),
        }.to_string()
    }

    /// Format message
    fn format_msg(&self) -> String {
        let re = Regex::new(r#"^request_id=(.*), client_ip_address=(.*), request_path="(.*)", status_code=(.*), elapsed_seconds=(.*), user_agent="(.*)"$"#);

        let mut access_log = AccessLogRecord::default();
        if re.is_ok() {
            for cap in re.unwrap().captures_iter(&self.msg) {
                access_log.request_id = cap.get(1).map_or("".to_owned(), |m| m.as_str().to_owned());
                access_log.client_ip_address = cap.get(2).map_or("".to_owned(), |m| m.as_str().to_owned());
                access_log.request_path = cap.get(3).map_or("".to_owned(), |m| m.as_str().to_owned());
                access_log.status_code = cap.get(4).map_or(0, |m| m.as_str().parse::<u16>().unwrap());
                access_log.elapsed_seconds = cap.get(5).map_or(0.0, |m| m.as_str().parse::<f32>().unwrap());
                access_log.user_agent = cap.get(6).map_or("".to_owned(), |m| m.as_str().to_owned());
            }
        }

        if access_log.is_valid() {
            format!("{} | {} | {} | {: >10}s | {} | {}",
                access_log.status_code.to_string().on_green().black(), // TODO: Customize color
                access_log.request_path, // TODO: Customize method color
                access_log.request_id,
                access_log.elapsed_seconds,
                access_log.client_ip_address,
                access_log.user_agent,
            )
        } else {
            self.msg.to_string()
        }
    }
}

// TODO: Only &str?
#[derive(Debug)]
struct AccessLogRecord {
    request_id: String,
    client_ip_address: String,
    request_path: String,
    status_code: u16,
    elapsed_seconds: f32,
    user_agent: String,
}

impl AccessLogRecord {
    fn is_valid(&self) -> bool {
        self.status_code >= 100 && self.status_code <= 599
    }
}

impl Default for AccessLogRecord {
    fn default() -> Self {
        Self {
            request_id: String::from(""),
            client_ip_address: String::from(""),
            request_path: String::from(""),
            status_code: 0,
            elapsed_seconds: 0.0,
            user_agent: String::from(""),
        }
    }
}
