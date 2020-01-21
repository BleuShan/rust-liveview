//! DOM representation.

use crate::Renderer;
pub use rust_liveview_codegen::Element;
use std::marker::PhantomData;
use v_htmlescape::escape;

/// A Node represents an rendered element in a DOM tree.
pub trait Node<T>: Send
where
    T: Send,
{
    /// The HTML/SVG Node name representing the Node
    fn node_name(&self) -> &'static str;
    /// Render into a Renderer.
    fn render(&self, renderer: Box<&mut dyn Renderer<Output = T>>)
    where
        T: Send;
}

/// A representation of a SVGElement or HTMLElement.
pub trait Element {
    /// The list of typed attribute name defined for
    /// the element.
    fn attribute_names(&self) -> &'static [&'static str];
}

/// A node representing text content.
#[derive(Clone, Debug)]
pub enum TextNode<T> {
    /// An unsafe text node used to render unescaped html content.
    Raw(String, PhantomData<T>),
    /// A safe text node variant used to render escaped html content.
    Safe(String, PhantomData<T>),
}

impl<T> TextNode<T>
where
    T: Send,
{
    /// Creates a new Raw TextNode
    pub fn raw<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::Raw(s.into(), PhantomData)
    }
    /// Creates a new Safe TextNode
    pub fn safe<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::Safe(s.into(), PhantomData)
    }
}

impl<T> Node<T> for TextNode<T>
where
    T: Send,
{
    fn node_name(&self) -> &'static str {
        "text"
    }

    fn render(&self, renderer: Box<&mut dyn Renderer<Output = T>>) {
        let text = match self {
            Self::Safe(s, _) => escape(s).to_string(),
            Self::Raw(s, _) => s.clone(),
        };
        renderer.text(text);
    }
}
