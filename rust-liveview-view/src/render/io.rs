//! IO rendering context facilities.

use super::context::Update::{
    self,
    *,
};
use crate::prelude::*;
use std::io::{
    BufWriter,
    IntoInnerError,
    Write,
};

/// Byte RenderContext
#[derive(Debug)]
pub struct BufWriterRenderContext<W>
where
    W: Write + Send + Sync,
{
    writer: BufWriter<W>,
    closing_tags: Vec<Vec<u8>>,
}

impl<W> BufWriterRenderContext<W>
where
    W: Write + Send + Sync,
{
    /// Creates a new BufWriterRenderContext with the default capacity
    pub fn new(inner: W) -> Self {
        Self {
            closing_tags: Vec::default(),
            writer: BufWriter::new(inner),
        }
    }
    /// Creates a new BufWriterRenderContext with the specified capacity
    pub fn with_capacity(capacity: usize, inner: W) -> Self {
        Self {
            closing_tags: Vec::default(),
            writer: BufWriter::with_capacity(capacity, inner),
        }
    }

    fn start_element(
        &mut self,
        node_name: &'static str,
        attributes: Vec<(&'static str, String)>,
    ) -> Result<()> {
        write!(self.writer, "<{}", node_name)?;
        for (key, value) in attributes {
            if value.is_empty() {
                write!(self.writer, " {}", key)?;
            } else {
                write!(self.writer, " {}=\"{}\"", key, value)?;
            }
        }
        Ok(())
    }
}

impl<W> RenderContext for BufWriterRenderContext<W>
where
    W: Write + Send + Sync,
{
    type Target = W;
    type IntoInnerError = IntoInnerError<BufWriter<W>>;

    fn update(&mut self, update: Update) -> Result<()> {
        match update {
            ElementOpen {
                node_name,
                attributes,
            } => {
                self.start_element(node_name, attributes)?;
                write!(self.writer, ">")?;
                let mut close_tag = Vec::default();
                write!(close_tag, "</{}>", node_name)?;
                self.closing_tags.push(close_tag);
            }
            ElementClose => {
                if let Some(tag) = self.closing_tags.pop() {
                    self.writer.write_all(&tag)?;
                }
            }
            ElementVoid {
                node_name,
                attributes,
            } => {
                self.start_element(node_name, attributes)?;
                write!(self.writer, " />")?;
            }
            Text(text) => {
                let slice: Vec<u8> = text.bytes().collect();
                self.writer.write_all(&slice)?;
            }
        };

        self.writer.flush()?;

        Ok(())
    }

    fn into_inner(self) -> Result<Self::Target, Self::IntoInnerError> {
        self.writer.into_inner()
    }
}
