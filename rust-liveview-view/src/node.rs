//! DOM representation.

use crate::{
    render::Renderer,
    render_context::RenderContext,
    result::Result,
};

/// A Node represents an rendered element in a DOM tree.
pub trait Node<C>
where
    Self: Send,
    C: RenderContext,
{
    /// The HTML/SVG Node name representing the Node
    fn node_name(&self) -> &'static str;
    /// Render into a Renderer.
    fn render(&self, renderer: &mut Renderer<'_, C>) -> Result<()>;
}

/// A representation of a SVGElement or HTMLElement.
pub trait Element<C>: Node<C>
where
    C: RenderContext,
{
    /// The list of typed attribute name defined for
    /// the element.
    fn attribute_names(&self) -> &'static [&'static str];
}
