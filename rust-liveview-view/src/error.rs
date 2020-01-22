//! Renderer errors.

use std::{
    backtrace::Backtrace,
    fmt,
    io,
    result::Result as StdResult,
};
use thiserror::Error;

/// Type alias for a Renderer result
pub type Result<T, E = Error> = StdResult<T, E>;

/// Encapsulate any error that might happen during rendering
#[derive(Error, Debug)]
pub enum Error {
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
