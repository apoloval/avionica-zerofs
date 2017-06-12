//
// Avionica ZeroFS
// Copyright (c) 2017 Alvaro Polo
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error::{Error as StdError};
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use std::result;

use log::LogLevelFilter;
use log4rs;
use log4rs::pattern::PatternLayout;
use serde;
use serde::{Deserialize, Deserializer};
use toml;


#[derive(Debug)]
pub struct Error(String);

pub type Result<T> = result::Result<T, Error>;

#[derive(Deserialize)]
pub struct LoggingSettings {
    #[serde(default = "LoggingSettings::default_logging_level",
            deserialize_with = "deserialize_log_level_filter")]
    pub level: LogLevelFilter,

    #[serde(default = "LoggingSettings::default_pattern_layout",
            deserialize_with = "deserialize_pattern_layout")]
    pub pattern: PatternLayout,

    #[serde(default = "LoggingSettings::default_logging_file")]
    pub file: String,
}

impl LoggingSettings {
    pub fn default_logging_level() -> LogLevelFilter {
        LogLevelFilter::Info
    }

    pub fn default_pattern_layout() -> PatternLayout {
        PatternLayout::new("%d{%Y/%m/%d %H:%M:%S.%f} - [%l] [%M]: %m").unwrap()
    }

    pub fn default_logging_file() -> String {
        "Modules/zerofs.log".to_string()
    }
}

impl Default for LoggingSettings {
    fn default() -> LoggingSettings {
        LoggingSettings {
            level: Self::default_logging_level(),
            pattern: Self::default_pattern_layout(),
            file: Self::default_logging_file(),
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

#[derive(Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub logging: LoggingSettings,
}

impl Settings {
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> io::Result<Settings> {
        let mut file = try!(fs::File::open(&path));
        let mut content = String::with_capacity(10*1024);
        file.read_to_string(&mut content)?;
        Self::from_toml(&content)
            .map_err(|_| io::Error::new(
                io::ErrorKind::InvalidData,
                format!("cannot read config from file '{:?}'", path.as_ref().as_os_str())))
    }

    pub fn from_toml(toml: &str) -> Result<Settings> {
        toml::from_str(toml).map_err(|e| Error(e.description().to_string()))
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            logging: LoggingSettings::default(),
        }
    }
}

fn deserialize_log_level_filter<'de, D>(deserializer: D) -> result::Result<LogLevelFilter, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    match s.as_ref() {
        "off" => Ok(LogLevelFilter::Off),
        "error" => Ok(LogLevelFilter::Error),
        "warn" => Ok(LogLevelFilter::Warn),
        "info" => Ok(LogLevelFilter::Info),
        "debug" => Ok(LogLevelFilter::Debug),
        "trace" => Ok(LogLevelFilter::Trace),
        other => Err(serde::de::Error::custom(format!("unknown log level filter {}", other))),
    }
}
fn deserialize_pattern_layout<'de, D>(deserializer: D) -> result::Result<PatternLayout, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    PatternLayout::new(&s).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {

    use log::LogLevelFilter;

    use super::*;

    #[test]
    fn should_load_defaults_from_empty_toml() {
        let s = Settings::from_toml("").unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Info);
    }

    #[test]
    fn should_load_logging_defaults_from_empty_section() {
        let s = Settings::from_toml(r#"
        	[logging]
        	"#).unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Info);
    }

    #[test]
    fn should_load_logging_level() {
        let s = Settings::from_toml(r#"
        	[logging]
        	level = "DEBUG"
        	"#).unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Debug);
        let s = Settings::from_toml(r#"
        	[logging]
        	level = "warn"
        	"#).unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Warn);
        let s = Settings::from_toml(r#"
        	[logging]
        	level = "Trace"
        	"#).unwrap();
        assert_eq!(s.logging.level, LogLevelFilter::Trace);
    }

    #[test]
    fn should_load_logging_pattern() {
        let s = Settings::from_toml(r#"
        	[logging]
        	pattern = "the-pattern"
        	"#).unwrap();
        assert_eq!(
            format!("{:?}", s.logging.pattern),
            r#"PatternLayout { pattern: [Text("the-pattern")] }"#);
    }

    #[test]
    fn should_load_logging_file() {
        let s = Settings::from_toml(r#"
        	[logging]
        	file = "/path/to/log/file"
        	"#).unwrap();
        assert_eq!(s.logging.file, "/path/to/log/file");
    }
}
