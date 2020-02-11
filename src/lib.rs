use log_hal::Log;
use log_hal::Level;
use core::fmt::Arguments;
use time::PrimitiveDateTime;
use colored::Colorize;

pub struct LogStd {
    tag: &'static str,
}

impl Log for LogStd {
    fn tag(&mut self, t: &'static str) {
        self.tag = t;
    }

    fn log(&self, level: Level, args: &Arguments) {
        let r = format!("{} [{}] {}", PrimitiveDateTime::now().format("%c"), self.tag, args);
        match level {
            Level::Info => println!("{}", r.white()),
            Level::Warn => println!("{}", r.yellow()),
            Level::Debug => println!("{}", r.green()),
            Level::Error => println!("{}", r.red()),
            Level::Panic => println!("{}", r.bright_red()),
        }
    }

}

impl Default for LogStd {
    fn default() -> Self {
        LogStd {
            tag: ""
        }
    }

}

#[test]
fn test() {
    let mut logger = LogStd::default();
    logger.tag("arch");
    logger.debug(&format_args!("{}", "hello"));
    logger.info(&format_args!("{}", "hello"));
    logger.warn(&format_args!("{}", "hello"));
    logger.error(&format_args!("{}", "hello"));
    // logger.panic(&format_args!("{}", "hello"));
}
