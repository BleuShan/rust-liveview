use super::*;
use std::{
    marker::PhantomData,
    str,
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
    context: ByteBufferRenderContext,
}

#[session]
impl ElementTests {
    #[theory]
    #[case(Optional{property: Some("test".to_owned()), _phantom: PhantomData}, "<optional property=\"test\"></optional>")]
    #[case(Optional{property: None, _phantom: PhantomData}, "<optional></optional>")]
    fn element_to_string_should_be_able_to_skip_none(
        mut self,
        element: Optional<ByteBufferRenderContext>,
        expected: &str,
    ) {
        let mut renderer = Renderer::from(&mut self.context);
        element.render(&mut renderer).should().be_ok();
        let buffer = self.context.buffer();
        let result = str::from_utf8(&buffer);
        result.should().yield_the_item(expected);
    }

    #[fact]
    fn element_to_string_should_not_render_skipped_fields(mut self) {
        let element = Skipped {
            is_test: true,
            body: "hello world!".to_owned(),
            _phantom: PhantomData,
        };

        let mut renderer = Renderer::from(&mut self.context);
        element.render(&mut renderer).should().be_ok();
        let buffer = self.context.buffer();
        let result = str::from_utf8(&buffer);
        result
            .should()
            .yield_the_item("<skipped is-test=\"true\"></skipped>");
    }
}
