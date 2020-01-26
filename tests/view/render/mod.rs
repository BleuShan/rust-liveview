use super::*;
use std::string;

mod element;
mod text;

struct RenderTests {
    context: BufWriterRenderContext<Vec<u8>>,
}

impl Default for RenderTests {
    fn default() -> Self {
        let context = BufWriterRenderContext::new(Vec::default());
        Self { context }
    }
}

impl RenderTests {
    #[inline]
    fn render<E>(mut self, element: E) -> Result<String, string::FromUtf8Error>
    where
        E: Render<BufWriterRenderContext<Vec<u8>>>,
    {
        let mut renderer = Renderer::from(&mut self.context);
        element.render(&mut renderer).should().be_ok();
        let buffer = self.context.into_inner().unwrap();
        String::from_utf8(buffer)
    }
}
