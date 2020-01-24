//! HTML elements.

use super::attributes::*;
use crate::prelude::*;
use std::marker::PhantomData;

/// Html Element
#[derive(Debug, Element)]
pub struct Html<T> {
    xlmns: Option<Uri>,
    _phantom: PhantomData<T>,
}
