//! Rendering facilities.

use crate::Node;
use v_htmlescape::escape;

/// A Renderer defines how a Node should be processed to an arbitrary data type.
pub trait Renderer: Send + Sync {
    /// Output type of the renderer.
    type Output: Send;

    /// Starts the rendering of a Node.
    fn start_node(&mut self, node_name: &'static str);
    /// Renders an Attribute of the currently rendered Node.
    fn attribute(&mut self, name: &'static str, value: String);
    /// Renders the children of the currently rendered Node.
    fn children(&mut self, nodes: Box<dyn Iterator<Item = Box<dyn Node<Self::Output>>>>);
    /// Renders some text content into the currently rendered Node.
    fn text(&mut self, text: String);
    /// Closes the Rendering of a Node.
    fn end_node(&mut self, self_closing: bool);
    /// Consumes the Renderer and yields its output.
    fn finish(self) -> Self::Output;
}

/// Renders Nodes to a String
#[derive(Debug, Default)]
pub struct StringRenderer {
    current_node_name: &'static str,
    current_child_node_name: &'static str,
    buffer: String,
    children: Vec<String>,
}

impl Renderer for StringRenderer {
    type Output = String;

    fn start_node(&mut self, node_name: &'static str) {
        self.buffer.push_str(&format!("<{}", node_name));
        self.current_node_name = node_name;
    }

    fn attribute(&mut self, name: &'static str, value: String) {
        self.buffer
            .push_str(&format!(" {}=\"{}\"", name, escape(&value)));
    }

    fn children(&mut self, _nodes: Box<dyn Iterator<Item = Box<dyn Node<Self::Output>>>>) {}

    fn text(&mut self, text: String) {
        if self.buffer.is_empty() {
            self.buffer.push_str(&text);
        } else {
            self.children.push(text);
        }
    }

    fn end_node(&mut self, self_closing: bool) {
        if self_closing {
            self.buffer.push_str("/>");
        } else {
            let closing_tag = format!("</{}>", self.current_node_name);
            if !self.buffer.ends_with('>') {
                self.buffer.push('>');
            }

            for child in self.children.iter() {
                self.buffer.push_str(child);
            }

            self.buffer.push_str(&closing_tag);
        }
    }

    fn finish(self) -> Self::Output {
        self.buffer
    }
}
