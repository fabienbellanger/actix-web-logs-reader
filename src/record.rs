use std::borrow::Cow;

use chrono::{DateTime, SecondsFormat, Utc};
use colored::Colorize;
use regex::Regex;
use serde::Deserialize;

pub const LEVELS: [&str; 6] = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR", "FATAL"];

#[derive(Deserialize, Debug)]
pub struct LogRecord<'a> {
    pub time: DateTime<Utc>,
    pub level: &'a str,
    pub file: Option<&'a str>,
    pub line: Option<usize>,
    pub msg: Cow<'a, str>,
}

impl<'a> LogRecord<'a> {
    /// Format a record
    pub fn format(&self) -> String {
        format!(
            "{} {} | {}{}\n",
            self.time
                .to_rfc3339_opts(SecondsFormat::Secs, true)
                .bright_black(),
            self.format_level(),
            self.format_file_line(),
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
        }
        .to_string()
    }

    /// Format file path and line number
    fn format_file_line(&self) -> String {
        let mut f = String::new();
        if self.file.is_some() {
            f.push_str(self.file.unwrap_or(""));
            if self.line.is_some() {
                f.push(':');
                f.push_str(&self.line.unwrap().to_string());
            }
            f.push_str(" | ");
        }

        f
    }

    /// Format message
    fn format_msg(&self) -> String {
        let re = Regex::new(
            r#"^request_id=(.*), client_ip_address=(.*), request_path="(.*)", status_code=(.*), elapsed_seconds=(.*), user_agent="(.*)"$"#,
        );

        let mut access_log = AccessLogRecord::default();
        if let Ok(re) = re {
            for cap in re.captures_iter(&self.msg) {
                access_log.request_id = cap.get(1).map_or("", |m| m.as_str());
                access_log.client_ip_address = cap.get(2).map_or("", |m| m.as_str());
                access_log.request_path = cap.get(3).map_or("", |m| m.as_str());
                access_log.status_code =
                    cap.get(4).map_or(0, |m| m.as_str().parse::<u16>().unwrap());
                access_log.elapsed_seconds = cap
                    .get(5)
                    .map_or(0.0, |m| m.as_str().parse::<f32>().unwrap());
                access_log.user_agent = cap.get(6).map_or("", |m| m.as_str());
            }
        }

        if access_log.is_valid() {
            format!(
                "{} | {} | {} | {: >10}s | {} | {}",
                access_log.format_status_code(),
                access_log.format_request_path(),
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

#[derive(Debug)]
struct AccessLogRecord<'a> {
    request_id: &'a str,
    client_ip_address: &'a str,
    request_path: &'a str,
    status_code: u16,
    elapsed_seconds: f32,
    user_agent: &'a str,
}

impl<'a> AccessLogRecord<'a> {
    /// Check if the log is valid
    fn is_valid(&self) -> bool {
        self.status_code >= 100 && self.status_code <= 599
    }

    /// Format HTTP status code
    fn format_status_code(&self) -> String {
        match self.status_code {
            c if c < 200 => c.to_string().cyan().bold(),
            c if c < 300 => c.to_string().green().bold(),
            c if c < 400 => c.to_string().purple().bold(),
            c if c < 500 => c.to_string().yellow().bold(),
            c if c < 600 => c.to_string().red().bold(),
            _ => "".clear(),
        }
        .to_string()
    }

    /// Format HTTP request path
    fn format_request_path(&self) -> String {
        let mut parts = self.request_path.split(' ');
        let http_method = parts.next().unwrap_or("");
        let http_uri = parts.next().unwrap_or("");
        let http_protocol = parts.next().unwrap_or("");

        if http_method.is_empty() && http_uri.is_empty() && http_protocol.is_empty() {
            self.request_path.to_string()
        } else {
            format!(
                "{} | {} | {}",
                Self::format_http_method(http_method),
                http_uri,
                http_protocol
            )
        }
    }

    /// Format HTTP method
    fn format_http_method(method: &str) -> String {
        match method {
            "GET" => "    GET".green(),
            "POST" => "   POST".blue(),
            "PUT" => "    PUT".yellow(),
            "PATCH" => "  PATCH".magenta(),
            "DELETE" => " DELETE".red(),
            "HEAD" => "   HEAD".clear(),
            "CONNECT" => "CONNECT".clear(),
            "OPTIONS" => "OPTIONS".clear(),
            "TRACE" => "  TRACE".clear(),
            _ => "".clear(),
        }
        .to_string()
    }
}

impl<'a> Default for AccessLogRecord<'a> {
    fn default() -> Self {
        Self {
            request_id: "",
            client_ip_address: "",
            request_path: "",
            status_code: 0,
            elapsed_seconds: 0.0,
            user_agent: "",
        }
    }
}
