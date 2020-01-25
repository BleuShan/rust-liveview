//! Prelude

pub use crate::{
    element::{
        declare_element,
        declare_elements,
        Element,
    },
    html,
    render::{
        Render,
        RenderContext,
        Renderer,
    },
    result::Result,
    text::{
        escape,
        TextNode,
    },
};
