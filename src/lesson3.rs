//! Lesson 3 exercises: `Result`, the `?` operator and custom errors.
//!
//! The exercise tests are marked `#[ignore]` so normal `cargo test` and CI
//! stay green. Run them while you work with:
//!
//!     cargo test lesson3 -- --ignored
//!
//! Replace each `todo!()` with a real implementation until its test passes.
//! Once an exercise passes, delete its `#[ignore]` line.
#![allow(dead_code, unused_variables)]

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

/// Exercise 1: parse both strings and return their sum.
/// Use `?` twice. Do not use `unwrap`.
pub fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    todo!("parse a and b with ?, then add them")
}

#[derive(Debug)]
pub enum ConfigError {
    Empty,
    BadNumber(ParseIntError),
    OutOfRange(i32),
    /// Exercise 2: ports 1-1023 are reserved.
    Reserved(u16),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Empty => write!(f, "port is empty"),
            ConfigError::BadNumber(e) => write!(f, "not a number: {e}"),
            ConfigError::OutOfRange(n) => write!(f, "{n} is not a valid port"),
            // Exercise 2: should print e.g. "80 is a reserved port".
            ConfigError::Reserved(p) => todo!("write the message"),
        }
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

/// Exercise 2: after the range check, return `Err(ConfigError::Reserved(port))`
/// when the port is below 1024.
pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(ConfigError::Empty);
    }
    let n: i32 = s.parse()?;
    if !(1..=65535).contains(&n) {
        return Err(ConfigError::OutOfRange(n));
    }
    Ok(n as u16)
}

/// Exercise 3: read the file at `path`, trim it, and parse it as an `i32`.
/// Use `std::fs::read_to_string(path)?` and `?` on the parse. The error type
/// is `Box<dyn Error>`, so both kinds of failure convert automatically.
pub fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
    todo!("read the file, trim, parse")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Worked examples from the lesson. These already pass.

    #[test]
    fn parse_port_accepts_valid_port() {
        assert_eq!(parse_port("8080").unwrap(), 8080);
    }

    #[test]
    fn parse_port_rejects_bad_input() {
        assert!(matches!(parse_port(""), Err(ConfigError::Empty)));
        assert!(matches!(parse_port("abc"), Err(ConfigError::BadNumber(_))));
        assert!(matches!(
            parse_port("70000"),
            Err(ConfigError::OutOfRange(70000))
        ));
    }

    #[test]
    fn display_messages() {
        assert_eq!(ConfigError::Empty.to_string(), "port is empty");
        assert_eq!(
            ConfigError::OutOfRange(70000).to_string(),
            "70000 is not a valid port"
        );
    }

    // Exercises. Remove `#[ignore]` once each one passes.

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_add_strings_ok() {
        assert_eq!(add_strings("2", "40").unwrap(), 42);
    }

    #[test]
    #[ignore = "exercise 1"]
    fn exercise1_add_strings_propagates_error() {
        assert!(add_strings("2", "x").is_err());
        assert!(add_strings("x", "2").is_err());
    }

    #[test]
    #[ignore = "exercise 2"]
    fn exercise2_reserved_port_is_rejected() {
        assert!(matches!(parse_port("80"), Err(ConfigError::Reserved(80))));
        assert_eq!(parse_port("1024").unwrap(), 1024);
    }

    #[test]
    #[ignore = "exercise 2"]
    fn exercise2_reserved_display() {
        assert_eq!(
            ConfigError::Reserved(80).to_string(),
            "80 is a reserved port"
        );
    }

    #[test]
    #[ignore = "exercise 3"]
    fn exercise3_read_number_from_file() {
        let path = std::env::temp_dir().join(format!("lesson3_{}.txt", std::process::id()));
        std::fs::write(&path, "42\n").unwrap();
        let result = read_number(path.to_str().unwrap());
        std::fs::remove_file(&path).unwrap();
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    #[ignore = "exercise 3"]
    fn exercise3_missing_file_is_an_error() {
        assert!(read_number("definitely/not/a/real/file.txt").is_err());
    }
}
