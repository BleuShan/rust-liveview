//! IO render context facilities

use crate::{
    prelude::*,
    render_context::Update,
};
use std::io::Write;

const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

/// Byte RenderContext
#[derive(Debug)]
pub struct ByteBufferRenderContext {
    attribute_insert_index: usize,
    buffer: Vec<u8>,
}

impl Default for ByteBufferRenderContext {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_SIZE)
    }
}

impl ByteBufferRenderContext {
    /// Creates a new ByteBufferRenderContext with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            attribute_insert_index: 0,
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Creates a clone of it's buffer
    pub fn buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }
}

impl RenderContext for ByteBufferRenderContext {
    fn send(&mut self, update: Update) -> Result<()> {
        match update {
            Update::CreateNode(node_name, self_closing) => {
                write!(self.buffer, "<{}", node_name)?;
                self.attribute_insert_index = self.buffer.len();
                if self_closing {
                    write!(self.buffer, "/>")?;
                } else {
                    write!(self.buffer, ">")?;
                    write!(self.buffer, "</{}>", node_name)?;
                }
            }
            Update::CreateTextNode(text) => self.buffer.extend(text.bytes()),
            Update::Attribute(name, value) => {
                let (head, tail) = self.buffer.split_at(self.attribute_insert_index);
                let mut buffer = head.to_owned();
                write!(buffer, " {}=\"{}\"", name, value)?;
                buffer.extend(tail);
                self.attribute_insert_index = buffer.len();
                self.buffer = buffer;
            }
        };

        Ok(())
    }
}
