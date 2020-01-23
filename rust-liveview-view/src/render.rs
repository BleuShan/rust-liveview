//! Rendering facilities.

use crate::{
    node::Node,
    render_context::{
        RenderContext,
        Update,
    },
    result::Result,
};

/// A Renderer process a Node into a sequence of update operations where the RenderContext.
#[derive(Debug)]
pub struct Renderer<'a, C>
where
    C: RenderContext,
{
    context: &'a mut C,
}

impl<'a, C> From<&'a mut C> for Renderer<'a, C>
where
    C: RenderContext,
{
    fn from(context: &'a mut C) -> Self {
        Self { context }
    }
}

impl<'a, C> Renderer<'a, C>
where
    C: RenderContext,
{
    /// Starts the rendering of a Node.
    pub fn node(&mut self, node_name: &'static str, self_closing: bool) -> Result<()> {
        self.context
            .send(Update::CreateNode(node_name, self_closing))
    }

    /// Renders an Attribute of the currently rendered Node.
    pub fn attribute(&mut self, name: &'static str, value: String) -> Result<()> {
        self.context.send(Update::Attribute(name, value))
    }

    /// Renders the children of the currently rendered Node.
    pub fn children(&mut self, _nodes: Box<dyn Iterator<Item = Box<dyn Node<C>>>>) -> Result<()> {
        Ok(())
    }

    /// Renders some text content into the currently rendered Node.
    pub fn text(&mut self, text: String) -> Result<()> {
        self.context.send(Update::CreateTextNode(text))
    }
}
