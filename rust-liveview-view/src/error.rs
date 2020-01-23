//! Error related type definitions.

use std::{
    backtrace::Backtrace,
    error::Error as StdError,
    fmt,
    io,
};
use thiserror::Error;

/// Global error type for this library.
#[derive(Error, Debug)]
pub enum Error {
    /// Generic error
    #[error("{source}")]
    Any {
        #[from]
        #[doc(hidden)]
        source: Box<dyn StdError>,
        #[doc(hidden)]
        backtrace: Backtrace,
    },
    /// Formatting Error
    #[error("{source}")]
    Fmt {
        #[from]
        #[doc(hidden)]
        source: fmt::Error,
        #[doc(hidden)]
        backtrace: Backtrace,
    },
    /// IO error
    #[error("{source}")]
    IO {
        #[from]
        #[doc(hidden)]
        source: io::Error,
        #[doc(hidden)]
        backtrace: Backtrace,
    },
}
