pub enum LogLevel{
    Info,
    Warning,
    Error
}


pub struct Logger {
    loglevel: LogLevel
}

impl Logger {
    pub fn new(loglevel: LogLevel) -> Self {
        Self {loglevel}
    }

    pub fn set_level(&mut self, loglevel: LogLevel) {
        self.loglevel = loglevel;
    }

    fn log(text: &str) {
        println!(" {} ", text);
    }

    fn info(&self, text: &str) {
        match self.loglevel {
            LogLevel::Info => {Logger::log(text)},
            _ => {}
        }
    }

    fn warn(&self, text: &str) {
        match self.loglevel {
            LogLevel::Info => {},
            LogLevel::Warning => {Logger::log(text)},
            LogLevel::Error => {Logger::log(text)},
        }
    }

    fn error(&self, text: &str) {
        Logger::log(text);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn log_levels() {
        let logger = Logger::new(LogLevel::Info);

        logger.info("This is an information.");
        logger.warn("This is a warning!")
    }
}
