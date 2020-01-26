use super::*;

const HTML: &str = "<div>test</div>";

#[session]
impl RenderTests {
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
