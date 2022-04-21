use std::fmt::{self, Debug, Display};



use anyhow::anyhow;
use serde::{de, ser};

#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(anyhow!("{}", msg))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(anyhow!("{}", msg))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new<E>(err: E) -> Self
    where
        E: std::error::Error,
    {
        Self(anyhow!("{}", err))
    }
}
