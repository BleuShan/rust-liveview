use super::*;
use std::marker::PhantomData;

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

#[session]
impl RenderTests {
    #[fact]
    fn element_rendering_should_render_tag_name_properly(self) {
        let element = SomeCustomElement {
            accesskey: None,
            class: None,
            id: None,
            lang: None,
            title: None,
            _c: PhantomData,
        };
        self.render(element)
            .should()
            .yield_the_item("<some-custom-element></some-custom-element>".to_owned());
    }

    #[theory]
    #[case(SelfClosing {
        accesskey: None,
        class: None,
        id: None,
        lang: None,
        title: None,
        property: Some(true),
        _c: PhantomData
    }, "<self-closing property=\"true\" />")]
    #[case(SelfClosing {
        accesskey: None,
        class: None,
        id: None,
        lang: None,
        title: None,
        property: None,
        _c: PhantomData
    }, "<self-closing />")]
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
    #[case(Optional {
        accesskey: None,
        class: None,
        id: None,
        lang: None,
        title: None,
        property: Some("test".to_owned()), 
        _c: PhantomData
    }, "<optional property=\"test\"></optional>")]
    #[case(Optional {
        accesskey: None,
        class: None,
        id: None,
        lang: None,
        title: None,
        property: None,
        _c: PhantomData
    }, "<optional></optional>")]
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
            accesskey: None,
            class: None,
            id: None,
            lang: None,
            title: None,
            body: Some("hello world!".to_owned()),
            _c: PhantomData,
        };

        self.render(element)
            .should()
            .yield_the_item("<skipped is-test=\"true\"></skipped>".to_owned());
    }
}
