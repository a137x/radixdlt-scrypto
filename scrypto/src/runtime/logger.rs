use radix_engine_interface::api::api::LoggerApi;
use radix_engine_interface::api::types::Level;
use sbor::rust::string::String;
use scrypto::engine::scrypto_env::ScryptoEnv;

/// A utility for logging messages.
#[derive(Debug)]
pub struct Logger {}

impl Logger {
    /// Emits a log to console.
    pub fn log(level: Level, message: String) {
        ScryptoEnv.emit_log(level, message).unwrap();
    }

    /// Emits a trace message.
    pub fn trace(message: String) {
        Self::log(Level::Trace, message);
    }

    /// Emits a debug message.
    pub fn debug(message: String) {
        Self::log(Level::Debug, message);
    }

    /// Emits an info message.
    pub fn info(message: String) {
        Self::log(Level::Info, message);
    }

    /// Emits a warn message.
    pub fn warn(message: String) {
        Self::log(Level::Warn, message);
    }

    /// Emits an error message.
    pub fn error(message: String) {
        Self::log(Level::Error, message);
    }
}
