use super::*;
use std::{
    marker::PhantomData,
    string,
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

#[derive(Debug, Element)]
#[element(self_closing)]
struct SelfClosing<T> {
    property: Option<bool>,
    _phantom: PhantomData<T>,
}

struct ElementTests {
    context: BufWriterRenderContext<Vec<u8>>,
}

impl Default for ElementTests {
    fn default() -> Self {
        let context = BufWriterRenderContext::new(Vec::default());
        Self { context }
    }
}

#[session]
impl ElementTests {
    #[inline]
    fn render<E>(mut self, element: E) -> Result<String, string::FromUtf8Error>
    where
        E: Element<BufWriterRenderContext<Vec<u8>>>,
    {
        let mut renderer = Renderer::from(&mut self.context);
        element.render(&mut renderer).should().be_ok();
        let buffer = self.context.into_inner().unwrap();
        String::from_utf8(buffer)
    }

    #[fact]
    fn element_rendering_should_render_tag_name_properly(self) {
        let element = SomeCustomElement {
            _phantom: PhantomData,
        };
        self.render(element)
            .should()
            .yield_the_item("<some-custom-element></some-custom-element>".to_owned());
    }

    #[theory]
    #[case(SelfClosing{property: Some(true), _phantom: PhantomData}, "<self-closing property=\"true\" />")]
    #[case(SelfClosing{property: None, _phantom: PhantomData}, "<self-closing />")]
    fn element_rendering_should_render_self_closing_tags_properly(
        self,
        element: SelfClosing<BufWriterRenderContext<Vec<u8>>>,
        expected: &str,
    ) {
        self.render(element)
            .should()
            .yield_the_item(expected.to_owned());
    }

    #[theory]
    #[case(Optional{property: Some("test".to_owned()), _phantom: PhantomData}, "<optional property=\"test\"></optional>")]
    #[case(Optional{property: None, _phantom: PhantomData}, "<optional></optional>")]
    fn element_rendering_should_be_able_to_skip_none(
        self,
        element: Optional<BufWriterRenderContext<Vec<u8>>>,
        expected: &str,
    ) {
        self.render(element)
            .should()
            .yield_the_item(expected.to_owned());
    }

    #[fact]
    fn element_rendering_should_not_render_skipped_fields(self) {
        let element = Skipped {
            is_test: true,
            body: "hello world!".to_owned(),
            _phantom: PhantomData,
        };

        self.render(element)
            .should()
            .yield_the_item("<skipped is-test=\"true\"></skipped>".to_owned());
    }
}
