use std::fmt::{self, Display};

/// The error type for aur-depends.
#[derive(Debug)]
pub enum Error {
    /// An error occurred in the alpm crate.
    Alpm(alpm::Error),
    /// An error occurred in the rua crate.
    Raur(raur::Error),
}

impl From<alpm::Error> for Error {
    fn from(e: alpm::Error) -> Error {
        Error::Alpm(e)
    }
}

impl From<raur::Error> for Error {
    fn from(e: raur::Error) -> Error {
        Error::Raur(e)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Alpm(e) => e.fmt(fmt),
            Error::Raur(e) => e.fmt(fmt),
        }
    }
}

impl std::error::Error for Error {}
