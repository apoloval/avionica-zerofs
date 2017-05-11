//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::result;

use log::LogLevelFilter;
use log4rs;
use log4rs::pattern::PatternLayout;
use rustc_serialize::*;
use toml;

const DEFAULT_LOGGING_LEVEL: LogLevelFilter = LogLevelFilter::Info;
const DEFAULT_LOGGING_PATTERN: &'static str = "%d{%Y/%m/%d %H:%M:%S.%f} - [%l] [%M]: %m";
const DEFAULT_LOGGING_FILE: &'static str = "Modules/zerofs.log";

pub enum Error {
    CannotParse,
    CannotDecode,
}

pub type Result<T> = result::Result<T, Error>;

pub struct LoggingSettings {
    pub level: LogLevelFilter,
    pub pattern: PatternLayout,
    pub file: String,
}

impl Decodable for LoggingSettings {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        let mut result = LoggingSettings::default();
        if let Ok(level_str) = d.read_struct_field("level", 0, |d| d.read_str()) {
            result.level = try!(level_str
                .parse()
                .map_err(|_| d.error(&format!("unknown log level '{}'", level_str))));
        }
        if let Ok(pattern) = d.read_struct_field("pattern", 0, |d| d.read_str()) {
            result.pattern = try!(PatternLayout::new(&pattern)
                .map_err(|_| d.error(&format!("invalid log pattern in '{}'", pattern))));
        }
        if let Ok(file) = d.read_struct_field("file", 0, |d| d.read_str()) {
            result.file = file;
        }
        Ok(result)
    }
}

impl Default for LoggingSettings {
    fn default() -> LoggingSettings {
        LoggingSettings {
            level: DEFAULT_LOGGING_LEVEL,
            pattern: PatternLayout::new(DEFAULT_LOGGING_PATTERN).unwrap(),
            file: DEFAULT_LOGGING_FILE.to_string(),
        }
    }
}

impl From<LoggingSettings> for log4rs::config::Config {
    fn from(settings: LoggingSettings) -> log4rs::config::Config {
        let log_path = Path::new(&settings.file);
        let file_appender = log4rs::appender::FileAppender::builder(log_path)
            .pattern(settings.pattern)
            .build()
            .unwrap();
        let main_appender = log4rs::config::Appender::builder("main".to_string(), Box::new(file_appender))
            .build();
        let root = log4rs::config::Root::builder(settings.level)
            .appender("main".to_string())
            .build();
        let config = log4rs::config::Config::builder(root)
            .appender(main_appender)
            .build()
            .unwrap();
        config
    }
}



pub struct Settings {
    pub logging: LoggingSettings,
}

impl Settings {
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> io::Result<Settings> {
        let mut file = try!(fs::File::open(&path));
        let mut content = String::with_capacity(10*1024);
        try!(file.read_to_string(&mut content));
        Self::from_toml(&content)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                format!("cannot read config from file '{:?}'", path.as_ref().as_os_str())))
    }

    pub fn from_toml(toml: &str) -> Result<Settings> {
        let mut table = try!(toml::Parser::new(toml).parse().ok_or(Error::CannotParse));
        let logging =  match table.remove("logging") {
            Some(section) => try!(toml::decode(section).ok_or(Error::CannotDecode)),
            None => LoggingSettings::default(),
        };
        Ok(Settings {
            logging: logging,
        })
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            logging: LoggingSettings::default(),
        }
    }
}


#[cfg(test)]
mod tests {

    use log::LogLevelFilter;

    use super::*;

    #[test]
    fn should_load_defaults_from_empty_toml() {
        let s = Settings::from_toml("").ok().unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Info);
    }

    #[test]
    fn should_load_logging_defaults_from_empty_section() {
        let s = Settings::from_toml(r#"
        	[logging]
        	"#).ok().unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Info);
    }

    #[test]
    fn should_load_logging_level() {
        let s = Settings::from_toml(r#"
        	[logging]
        	level = "DEBUG"
        	"#).ok().unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Debug);
        let s = Settings::from_toml(r#"
        	[logging]
        	level = "warn"
        	"#).ok().unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Warn);
        let s = Settings::from_toml(r#"
        	[logging]
        	level = "Trace"
        	"#).ok().unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Trace);
    }

    #[test]
    fn should_load_logging_pattern() {
        let s = Settings::from_toml(r#"
        	[logging]
        	pattern = "the-pattern"
        	"#).ok().unwrap();
        assert_eq!(
        format!("{:?}", s.logging.pattern),
        r#"PatternLayout { pattern: [Text("the-pattern")] }"#);
    }

    #[test]
    fn should_load_logging_file() {
        let s = Settings::from_toml(r#"
        	[logging]
        	file = "/path/to/log/file"
        	"#).ok().unwrap();
        assert_eq!(s.logging.file, "/path/to/log/file");
    }
}
