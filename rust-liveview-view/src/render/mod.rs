//! Render facilities

mod context;
pub mod io;
mod renderer;

use crate::result::Result;
pub use context::RenderContext;
pub use renderer::Renderer;

/// The interface by which an object can be process by a Renderer instance.
pub trait Render<C>
where
    Self: Send,
    C: RenderContext,
{
    /// Render the object into a Renderer's RenderContext.
    fn render(&self, renderer: &mut Renderer<'_, C>) -> Result<()>;
}
