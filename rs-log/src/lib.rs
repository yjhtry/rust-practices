use std::str::FromStr;

/// Represents the log level.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Level {
    Info,
    Warning,
    Error,
}

impl FromStr for Level {
    type Err = ();

    /// Parses a string into a `Level` enum variant.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Self::Err>` - The parsed `Level` variant or an error if parsing fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Level::Info),
            "1" => Ok(Level::Warning),
            "2" => Ok(Level::Error),
            _ => Err(()),
        }
    }
}

/// Represents a logger.
/// ```
/// use log::{Level, Log};
/// let mut log = Log::new();
/// log.set_level(Level::Warning);
/// log.info("This is an info message");
/// log.warning("This is a warning message");
/// log.error("This is an error message");
/// ```
pub struct Log {
    level: Level,
    output: Option<Box<dyn std::io::Write>>,
}

impl Log {
    /// Creates a new `Log` instance with the default log level and output set to stdout.
    ///
    /// # Returns
    ///
    /// * `Self` - The new `Log` instance.
    pub fn new() -> Self {
        let level: Level = std::env::var("LOG_LEVEL")
            .unwrap_or("0".to_string())
            .parse()
            .unwrap();

        Log {
            level,
            output: Some(Box::new(std::io::stdout())),
        }
    }

    /// Sets the log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level to set.
    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    /// Sets the log output.
    ///
    /// # Arguments
    ///
    /// * `output` - The log output to set.
    pub fn set_output(&mut self, output: Box<dyn std::io::Write>) {
        self.output = Some(output);
    }

    /// Writes an info message to the log if the log level allows it.
    ///
    /// # Arguments
    ///
    /// * `message` - The info message to write.
    pub fn info(&mut self, message: &str) {
        if Level::Info >= self.level {
            if let Some(ref mut output) = self.output {
                writeln!(output, "INFO: {}", message).unwrap();
            }
        }
    }

    /// Writes a warning message to the log if the log level allows it.
    ///
    /// # Arguments
    ///
    /// * `message` - The warning message to write.
    pub fn warning(&mut self, message: &str) {
        if Level::Warning >= self.level {
            if let Some(ref mut output) = self.output {
                writeln!(output, "WARNING: {}", message).unwrap();
            }
        }
    }

    /// Writes an error message to the log if the log level allows it.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to write.
    pub fn error(&mut self, message: &str) {
        if Level::Error >= self.level {
            if let Some(ref mut output) = self.output {
                writeln!(output, "ERROR: {}", message).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_write_to_stdout() {
        let mut log = Log::new();

        log.set_level(Level::Warning);

        log.info("This is an info message");
        log.warning("This is a warning message");
        log.error("This is an error message");
    }

    #[test]
    fn test_log_write_to_file() {
        let mut log = Log::new();

        log.set_level(Level::Warning);

        let file = std::fs::File::create("test.log").unwrap();
        log.set_output(Box::new(file));

        log.info("This is an info message");
        log.warning("This is a warning message");
        log.error("This is an error message");
    }

    #[test]
    fn test_set_level_by_env_var() {
        std::env::set_var("LOG_LEVEL", "2");

        let log = Log::new();

        assert_eq!(log.level, Level::Error);
    }
}
