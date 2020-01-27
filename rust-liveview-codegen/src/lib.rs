#![warn(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub
)]
#![feature(
    associated_type_defaults,
    backtrace,
    box_patterns,
    box_syntax,
    error_iter,
    never_type,
    pattern,
    stdsimd,
    trait_alias,
    type_alias_impl_trait,
    try_blocks,
    try_trait
)]
#![recursion_limit = "512"]

#[macro_use]
mod macros;
cfg_common! {
    mod helpers;
    use proc_macro::TokenStream;
    use proc_macro_error::proc_macro_error;
    use syn::parse_macro_input;
}

cfg_runtime! {
    mod runtime;
    use syn::{
        AttributeArgs,
        ItemFn,
    };
    #[proc_macro_error]
    #[proc_macro_attribute]
    pub fn runtime_entrypoint_main(args: TokenStream, input: TokenStream) -> TokenStream {
        let attr_args = parse_macro_input!(args as AttributeArgs);
        let item = parse_macro_input!(input as ItemFn);
        runtime::entrypoint::set_fn_dummy(&item);
        runtime::entrypoint::Attribute::from(attr_args).generate(item)
    }

    #[proc_macro_error]
    #[proc_macro_attribute]
    pub fn runtime_entrypoint_test(args: TokenStream, input: TokenStream) -> TokenStream {
        let attr_args = parse_macro_input!(args as AttributeArgs);
        let item = parse_macro_input!(input as ItemFn);
        runtime::entrypoint::set_fn_dummy(&item);
        runtime::entrypoint::Attribute::from(attr_args).generate(item)
    }
}

cfg_view! {
    mod view;
    #[proc_macro_error]
    #[proc_macro_derive(Element, attributes(element))]
    pub fn view_element(input: TokenStream) -> TokenStream {
        let item = parse_macro_input!(input as view::Element);
        TokenStream::from(item)
    }

    #[proc_macro_error]
    #[proc_macro]
    pub fn view_define_element(input: TokenStream) -> TokenStream {
        let declaration = parse_macro_input!(input as view::ElementDefinition);
        TokenStream::from(declaration)
    }

    #[proc_macro_error]
    #[proc_macro]
    pub fn view_define_elements(input: TokenStream) -> TokenStream {
        let declaration = parse_macro_input!(input as view::ElementDefinitions);
        TokenStream::from(declaration)
    }
}
