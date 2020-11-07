use serde;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub enum Error {
    Custom(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<super::de::Error> for Error {
    fn from(e: super::de::Error) -> Self {
        Error::Custom(e.to_string())
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        "INI serialization error"
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
