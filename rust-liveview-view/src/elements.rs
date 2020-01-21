//! Html elements.

use crate::{
    Element,
    Node,
    Renderer,
};
use std::marker::PhantomData;

/// Html Element
#[derive(Debug, Element)]
pub struct Html<T> {
    xlmns: Option<String>,
    _phantom: PhantomData<T>,
}
