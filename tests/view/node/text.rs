use super::*;

const HTML: &str = "<div>test</div>";
use std::string;

struct TextNodeTests {
    context: BufWriterRenderContext<Vec<u8>>,
}

impl Default for TextNodeTests {
    fn default() -> Self {
        let context = BufWriterRenderContext::new(Vec::default());
        Self { context }
    }
}

#[session]
impl TextNodeTests {
    #[inline]
    fn render(
        mut self,
        node: TextNode<BufWriterRenderContext<Vec<u8>>>,
    ) -> Result<String, string::FromUtf8Error> {
        let mut renderer = Renderer::from(&mut self.context);
        node.render(&mut renderer).should().be_ok();
        let buffer = self.context.into_inner().unwrap();
        String::from_utf8(buffer)
    }

    #[theory]
    #[case(TextNode::raw(HTML), HTML.to_owned())]
    #[case(TextNode::safe(HTML), escape(HTML).to_string())]
    fn text_node_rendering_should_work_as_expected(
        self,
        node: TextNode<BufWriterRenderContext<Vec<u8>>>,
        expected: String,
    ) {
        self.render(node).should().yield_the_item(expected);
    }
}
