//! The view rendering prelude

pub use crate::{
    attributes,
    element::{
        define_element,
        define_elements,
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
