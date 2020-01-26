//! Error related type definitions.

use rust_liveview_common::thiserror::{
    self,
    Error,
};
use std::{
    backtrace::Backtrace,
    error::Error as StdError,
    fmt,
    io,
};

/// Global error type for this library.
#[derive(Error, Debug)]
pub enum Error {
    /// Generic error.
    #[error("{source}")]
    Any {
        #[from]
        #[doc(hidden)]
        source: Box<dyn StdError>,
        #[doc(hidden)]
        backtrace: Backtrace,
    },
    /// Custom Error.
    #[error("{0}")]
    Custom(String),
    /// Formatting Error.
    #[error("{source}")]
    Fmt {
        #[from]
        #[doc(hidden)]
        source: fmt::Error,
        #[doc(hidden)]
        backtrace: Backtrace,
    },
    /// IO error.
    #[error("{source}")]
    IO {
        #[from]
        #[doc(hidden)]
        source: io::Error,
        #[doc(hidden)]
        backtrace: Backtrace,
    },
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Custom(s)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::from(s.to_owned())
    }
}
