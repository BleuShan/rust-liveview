use super::*;
use render::StringRenderer;

const HTML: &str = "<div>test</div>";

#[derive(Default)]
struct TextNodeTests {
    renderer: StringRenderer,
}

#[session]
impl TextNodeTests {
    #[fact]
    fn a_raw_text_node_should_leave_its_content_unescaped(mut self) {
        TextNode::raw(HTML).render(Box::new(&mut self.renderer));
        self.renderer.finish().should().be_equal_to(HTML);
    }

    #[fact]
    fn a_safe_text_node_should_have_its_content_escaped(mut self) {
        TextNode::safe(HTML).render(Box::new(&mut self.renderer));
        self.renderer
            .finish()
            .should()
            .not()
            .be_empty()
            .and_should()
            .not()
            .be_equal_to(HTML);
    }
}
