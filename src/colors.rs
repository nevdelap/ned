use ned_error::StringError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Colors {
    Auto,
    Always,
    Never,
    Off,
}

impl FromStr for Colors {
    type Err = StringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Colors::Auto),
            "auto" => Ok(Colors::Auto),
            "always" => Ok(Colors::Always),
            "never" => Ok(Colors::Never),
            _ => Err(StringError {
                err: format!("invalid colors option {}", s),
            }),
        }
    }
}
