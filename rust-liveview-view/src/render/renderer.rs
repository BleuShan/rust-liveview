//! Renderer facilities.

use super::{
    context::{
        RenderContext,
        Update::*,
    },
    Render,
};
use crate::result::Result;
use rust_liveview_util::From;

/// A Renderer process a Render object into a sequence of update operations
/// for its RenderContext to process.
#[derive(Debug, From)]
pub struct Renderer<'a, C>
where
    C: RenderContext,
{
    context: &'a mut C,
}

impl<'a, C> Renderer<'a, C>
where
    C: RenderContext,
{
    /// Renders the children of the currently rendered Node.
    pub fn children(
        &mut self,
        _children: Box<dyn Iterator<Item = Box<dyn Render<C>>>>,
    ) -> Result<()> {
        Ok(())
    }

    /// Render the open tag of an Element.
    pub fn element_open(
        &mut self,
        node_name: &'static str,
        attributes: Vec<(&'static str, String)>,
    ) -> Result<()> {
        self.context.update(ElementOpen {
            node_name,
            attributes,
        })
    }

    /// Renders a self-closing element
    pub fn element_void(
        &mut self,
        node_name: &'static str,
        attributes: Vec<(&'static str, String)>,
    ) -> Result<()> {
        self.context.update(ElementVoid {
            node_name,
            attributes,
        })
    }

    /// Render the close tag of an Element
    pub fn element_close(&mut self) -> Result<()> {
        self.context.update(ElementClose)
    }

    /// Renders some text content into the currently rendered Element.
    pub fn text(&mut self, text: String) -> Result<()> {
        self.context.update(Text(text))
    }
}
