//! The config module contains the log configuration options for the config writer.
//! 

pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

pub enum LogOutput {
    Stdout,
    Sterr,
    File(String),
}

/// This struct contains the configuration options for controlling logging.
/// # Example
/// ```rust
/// use oci_config::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Create a new 'Logging' struct with custom values.
/// # Example
/// ```rust
/// use oci_config::config::{Logging, LogLevel, LogOutput};
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