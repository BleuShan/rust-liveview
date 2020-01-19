//! DOM representation.

pub use rust_liveview_codegen::Element;

use std::fmt::{
    self,
    Display,
};
use v_htmlescape::escape;

#[cfg(test)]
mod tests;

/// A Node represents an rendered element in a DOM tree.
pub trait Node: Display + Send {
    /// The HTML/SVG Node name representing the Node
    const NODE_NAME: &'static str;
}

/// A representation of a SVGElement or HTMLElement.
pub trait Element: Node {
    /// The list of typed attribute name defined for
    /// the element.
    const ATTRIBUTE_NAMES: &'static [&'static str];
}

/// A node representing text content.
#[derive(Clone, Debug)]
pub enum TextNode {
    /// An unsafe text node used to render unescaped html content.
    Raw(String),
    /// A safe text node variant used to render escaped html content.
    Safe(String),
}

impl TextNode {
    /// Creates a new Raw TextNode
    pub fn raw<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::Raw(s.into())
    }
    /// Creates a new Safe TextNode
    pub fn safe<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self::Safe(s.into())
    }
}

impl Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Safe(s) => {
                let escaped = escape(s);
                escaped.fmt(f)
            }
            Self::Raw(s) => f.write_str(s),
        }
    }
}

impl Node for TextNode {
    const NODE_NAME: &'static str = "text";
}
