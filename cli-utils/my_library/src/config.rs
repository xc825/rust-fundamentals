//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use my_library::config::Logging;
//! let config = Logging::new();
//! ```
//!
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// impl LogLevel {
//     pub fn Copy(&self) -> LogLevel {
//         match self {
//             _ => *self // Implement the Copy trait for LogLevel
//         }
//     }
// }

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use my_library::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct with custom values:
/// ```
/// use my_library::config::{Logging, LogLevel, LogOutput};
/// let mut config = Logging::new();
/// config.set_enabled(true);
/// config.set_level(LogLevel::Debug);
/// config.set_destination(LogOutput::File("log.txt".to_string()));
/// ```
pub struct Logging {
    enabled: bool,
    level: LogLevel,
    destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
    // getters
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }

    pub fn get_destination(&self) -> &LogOutput {
        &self.destination
    }

    // setters
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }

}