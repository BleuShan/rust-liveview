#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(
    associated_type_defaults,
    box_patterns,
    box_syntax,
    error_iter,
    never_type,
    pattern,
    slice_patterns,
    stdsimd,
    trait_alias,
    type_alias_impl_trait,
    try_blocks,
    try_trait
)]
#![recursion_limit = "512"]

mod helpers;
mod runtime;
mod view;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{
    parse_macro_input,
    AttributeArgs,
    DeriveInput,
    ItemFn,
};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn runtime(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as ItemFn);
    helpers::set_fn_dummy(&item);
    runtime::Attribute::from(attr_args).generate(item)
}

#[proc_macro_error]
#[proc_macro_derive(Element, attributes(element))]
pub fn element(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    view::Element::from(item).generate()
}
