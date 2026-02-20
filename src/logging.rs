use serde_json::json;
use std::sync::OnceLock;

/// Logging format - plain text or JSON
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogFormat {
    /// Plain text format: "[LEVEL] message"
    Plain,
    /// JSON format: {"level":"LEVEL","message":"..."}
    Json,
}

/// Log levels
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Returns the string representation of the log level
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Whether systemd journal is detected (via JOURNAL_STREAM env var)
fn is_systemd_journal() -> bool {
    std::env::var("JOURNAL_STREAM").is_ok()
}

static LOG_FORMAT: OnceLock<LogFormat> = OnceLock::new();
static SYSTEMD_MODE: OnceLock<bool> = OnceLock::new();

/// Initialize the logging system with the specified format
pub fn init_logging(format: LogFormat) {
    LOG_FORMAT.set(format).ok();
    SYSTEMD_MODE.set(is_systemd_journal()).ok();
}

/// Get the current log format
fn get_format() -> LogFormat {
    *LOG_FORMAT.get_or_init(|| LogFormat::Plain)
}

/// Check if running under systemd
fn is_systemd() -> bool {
    *SYSTEMD_MODE.get_or_init(|| is_systemd_journal())
}

/// Internal logging function
fn log_internal(level: LogLevel, message: &str) {
    let format = get_format();
    let systemd = is_systemd();

    let output = match format {
        LogFormat::Plain => {
            format!("[{}] {}", level.as_str(), message)
        }
        LogFormat::Json => json!({
            "level": level.as_str(),
            "message": message
        })
        .to_string(),
    };

    // Routing: INFO/DEBUG -> stdout, WARN/ERROR -> stderr
    // Under systemd: all goes to stdout (journald handles routing)
    if systemd {
        println!("{}", output);
    } else {
        match level {
            LogLevel::Debug | LogLevel::Info => println!("{}", output),
            LogLevel::Warn | LogLevel::Error => eprintln!("{}", output),
        }
    }
}

/// Log a debug message
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logging::log_debug_impl(&format!($($arg)*))
    };
}

/// Log an info message
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logging::log_info_impl(&format!($($arg)*))
    };
}

/// Log a warning message
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logging::log_warn_impl(&format!($($arg)*))
    };
}

/// Log an error message
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logging::log_error_impl(&format!($($arg)*))
    };
}

/// Internal implementation for log_debug macro
pub fn log_debug_impl(message: &str) {
    log_internal(LogLevel::Debug, message);
}

/// Internal implementation for log_info macro
pub fn log_info_impl(message: &str) {
    log_internal(LogLevel::Info, message);
}

/// Internal implementation for log_warn macro
pub fn log_warn_impl(message: &str) {
    log_internal(LogLevel::Warn, message);
}

/// Internal implementation for log_error macro
pub fn log_error_impl(message: &str) {
    log_internal(LogLevel::Error, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_format_variants() {
        assert_eq!(LogLevel::Debug.as_str(), "DEBUG");
        assert_eq!(LogLevel::Info.as_str(), "INFO");
        assert_eq!(LogLevel::Warn.as_str(), "WARN");
        assert_eq!(LogLevel::Error.as_str(), "ERROR");
    }

    #[test]
    fn test_init_logging() {
        // Should not panic
        init_logging(LogFormat::Plain);
        init_logging(LogFormat::Json);
    }

    #[test]
    fn test_log_impl_functions() {
        // These should not panic
        log_debug_impl("test debug");
        log_info_impl("test info");
        log_warn_impl("test warn");
        log_error_impl("test error");
    }
}
