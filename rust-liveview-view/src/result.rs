//! Result extensions

use crate::error::Error;
use std::result::Result as StdResult;

/// Convenience Result type alias.
pub type Result<T, E = Error> = StdResult<T, E>;
