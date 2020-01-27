//! DOM Element representation.

use crate::render::{
    Render,
    RenderContext,
};
pub use rust_liveview_codegen::{
    view_define_element as define_element,
    view_define_elements as define_elements,
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
