use super::*;

const HTML: &str = "<div>test</div>";

#[fact]
fn a_raw_text_node_should_leave_its_content_unescaped() {
    TextNode::raw(HTML).to_string().should().be_equal_to(HTML);
}

#[fact]
fn a_safe_text_node_should_have_its_content_escaped() {
    TextNode::safe(HTML)
        .to_string()
        .should()
        .not()
        .be_equal_to(HTML);
}
