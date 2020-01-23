//! RenderContext facilities.

use crate::result::Result;

/// Defines any updates exchanged between the render and it's context.
#[derive(Debug)]
pub enum Update {
    /// Creates a node
    CreateNode(&'static str, bool),
    /// Create a text node
    CreateTextNode(String),
    /// Renders an attribute
    Attribute(&'static str, String),
}

/// A RenderContext encapsulates the IO bits a Renderer operations.
/// It writes updates produced by a render into a rendering target.
pub trait RenderContext
where
    Self: Send + Sync,
{
    /// Output target type of the Renderer operations
    type Target: Send + Sync = [u8];

    /// Writes an update into its underlying target
    fn send(&mut self, update: Update) -> Result<()>;
}
