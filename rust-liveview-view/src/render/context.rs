//! RenderContext facilities.

use crate::{
    error::Error,
    result::Result,
};

/// Defines any updates exchanged between the render and it's context.
#[derive(Debug)]
pub enum Update {
    /// Creates an Element
    ElementOpen {
        node_name: &'static str,
        attributes: Vec<(&'static str, String)>,
    },
    /// Creates a self-closing Element
    ElementVoid {
        node_name: &'static str,
        attributes: Vec<(&'static str, String)>,
    },
    /// Closes the previously opened Element
    ElementClose,
    /// Create a text node
    Text(String),
}

/// A RenderContext encapsulates the IO bits a Renderer operations.
/// It writes updates produced by a render into a rendering target.
pub trait RenderContext
where
    Self: Send + Sync,
{
    /// Output target type of the Renderer operations.
    type Target: Send + Sync = [u8];
    /// Error type used by the into_inner method.
    type IntoInnerError = Error;
    /// Updates its underlying target.
    fn update(&mut self, update: Update) -> Result<()>;
    /// Converts the context into the underlying target.
    fn into_inner(self) -> Result<Self::Target, Self::IntoInnerError>;
}
