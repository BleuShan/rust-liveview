//! DOM Element representation.

use crate::render::{
    Render,
    RenderContext,
};
pub use rust_liveview_codegen::{
    define_element,
    define_elements,
    Element,
};

/// A representation of a SVGElement or HTMLElement.
pub trait Element<C>: Render<C>
where
    C: RenderContext,
{
    /// The list of attributes key/value pairs defined on the element.
    fn attributes(&self) -> Vec<(&'static str, String)>;
}
