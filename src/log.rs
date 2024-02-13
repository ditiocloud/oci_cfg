//! The log module provides options for logging configurations. It contains the 'LogLevel' and 'LogOutput' enums, and the 'Logging' struct.
//! The 'LogLevel' enum represents different levels for logging.The 'LogOutput' enum represents optional destinations for logging.
//! The 'Logging' struct contains the configuration options for controlling logging.

/// represents different levels for logging.
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

/// represents optional destinations for logging.
pub enum LogOutput {
    Stdout,
    Sterr,
    File(String),
}

/// contains the configuration options for controlling logging.
/// # Example
/// ```rust
/// use oci_config_writer::log::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Create a new 'Logging' struct with custom values.
/// # Example
/// ```rust
/// use oci_config_writer::log::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///    enabled: true,
///    level: LogLevel::Debug,
///    destination: LogOutput::File("log.txt".to_string()),
/// };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}