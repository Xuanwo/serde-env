use std::fmt::{self, Debug, Display};

use serde::{de, ser};

/// Errors returned by serde-env.
#[derive(Debug)]
pub struct Error(String);

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn new<E>(err: E) -> Self
    where
        E: std::error::Error,
    {
        Self(err.to_string())
    }

    pub(crate) fn from_str(err: &str) -> Self {
        Self(err.to_string())
    }
}
