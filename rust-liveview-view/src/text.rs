//! Text rendering facilities.

use crate::{
    render::{
        Render,
        RenderContext,
        Renderer,
    },
    result::Result,
};
use std::marker::PhantomData;
pub use v_htmlescape::escape;

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

impl<T> Render<T> for TextNode<T>
where
    T: RenderContext,
{
    fn render(&self, renderer: &mut Renderer<'_, T>) -> Result<()> {
        let text = match self {
            Self::Safe(s, _) => escape(s).to_string(),
            Self::Raw(s, _) => s.clone(),
        };
        renderer.text(text)?;
        Ok(())
    }
}
