use super::*;
use std::{
    marker::PhantomData,
    string,
};

define_elements! {
    Optional {
        property: String
    },
    Skipped {
        is_test: bool,
        #[element(skip)]
        body: String,
    },
    SomeCustomElement,
    #[element(self_closing)]
    SelfClosing {
        property: bool
    }
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
}

#[session]
impl ElementTests {
    #[fact]
    fn element_rendering_should_render_tag_name_properly(self) {
        let element = SomeCustomElement { _c: PhantomData };
        self.render(element)
            .should()
            .yield_the_item("<some-custom-element></some-custom-element>".to_owned());
    }

    #[theory]
    #[case(SelfClosing{property: Some(true), _c: PhantomData}, "<self-closing property=\"true\" />")]
    #[case(SelfClosing{property: None, _c: PhantomData}, "<self-closing />")]
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
    #[case(Optional{property: Some("test".to_owned()), _c: PhantomData}, "<optional property=\"test\"></optional>")]
    #[case(Optional{property: None, _c: PhantomData}, "<optional></optional>")]
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
            is_test: Some(true),
            body: Some("hello world!".to_owned()),
            _c: PhantomData,
        };

        self.render(element)
            .should()
            .yield_the_item("<skipped is-test=\"true\"></skipped>".to_owned());
    }
}
