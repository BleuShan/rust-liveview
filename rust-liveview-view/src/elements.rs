//! Builtin elements
use crate::dom::{
    Element,
    Node,
};

/// Html Element
#[derive(Debug, Element)]
pub struct Html {
    xlmns: Option<String>,
}
