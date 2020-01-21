use super::*;
use std::marker::PhantomData;
use view::{
    render::StringRenderer,
    Element,
};

#[derive(Debug, Element)]
struct Optional<T> {
    property: Option<String>,
    _phantom: PhantomData<T>,
}

#[derive(Debug, Element)]
struct Skipped<T> {
    is_test: bool,
    #[element(skip)]
    body: String,
    _phantom: PhantomData<T>,
}

#[derive(Debug, Element)]
struct SomeCustomElement<T> {
    _phantom: PhantomData<T>,
}
#[derive(Default)]
struct ElementTests {
    renderer: StringRenderer,
}

#[session]
impl ElementTests {
    #[theory]
    #[case(Optional{property: Some("test".to_owned()), _phantom: PhantomData}, "<optional property=\"test\"></optional>")]
    #[case(Optional{property: None, _phantom: PhantomData}, "<optional></optional>")]
    fn element_to_string_should_be_able_to_skip_none(
        mut self,
        element: Optional<String>,
        expected: &str,
    ) {
        element.render(Box::new(&mut self.renderer));
        self.renderer.finish().should().be_equal_to(expected);
    }

    #[fact]
    fn element_to_string_should_not_render_skipped_fields(mut self) {
        let value = Skipped {
            is_test: true,
            body: "hello world!".to_owned(),
            _phantom: PhantomData,
        };

        value.render(Box::new(&mut self.renderer));
        self.renderer
            .finish()
            .should()
            .be_equal_to("<skipped is-test=\"true\"></skipped>".to_owned());
    }
}
