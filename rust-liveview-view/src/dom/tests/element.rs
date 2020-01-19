use super::*;

#[derive(Debug, Element)]
struct Optional {
    property: Option<String>,
}

#[derive(Debug, Element)]
struct Skipped {
    is_test: bool,
    #[element(skip)]
    body: String,
}

#[derive(Debug, Element)]
struct SomeCustomElement;

#[theory]
#[case(Optional{property: Some("test".to_owned())}, "<optional property=\"test\"></optional>")]
#[case(Optional{property: None}, "<optional></optional>")]
fn element_to_string_should_be_able_to_skip_none(element: Optional, expected: &str) {
    element.to_string().should().be_equal_to(expected);
}

#[fact]
fn element_to_string_should_not_render_skipped_fields() {
    let value = Skipped {
        is_test: true,
        body: "hello world!".to_owned(),
    }
    .to_string();

    value
        .should()
        .be_equal_to("<skipped is-test=\"true\"></skipped>".to_owned());
}
