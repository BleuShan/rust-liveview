use super::*;

const HTML: &str = "<div>test</div>";
use std::str;

#[derive(Default)]
struct TextNodeTests {
    context: ByteBufferRenderContext,
}

#[session]
impl TextNodeTests {
    #[fact]
    fn a_raw_text_node_should_leave_its_content_unescaped(mut self) {
        let mut renderer = Renderer::from(&mut self.context);
        TextNode::raw(HTML).render(&mut renderer).should().be_ok();
        let buffer = self.context.buffer();
        let result = str::from_utf8(&buffer);
        result.should().yield_the_item(HTML);
    }

    #[fact]
    fn a_safe_text_node_should_have_its_content_escaped(mut self) {
        let mut renderer = Renderer::from(&mut self.context);
        TextNode::safe(HTML).render(&mut renderer).should().be_ok();
        let buffer = self.context.buffer();
        let result = str::from_utf8(&buffer);
        result.should().not().yield_the_item(HTML);
    }
}
