#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(
    associated_type_defaults,
    box_patterns,
    box_syntax,
    error_iter,
    never_type,
    proc_macro_diagnostic,
    stdsimd,
    trait_alias,
    type_alias_impl_trait,
    try_blocks,
    try_trait
)]
#![recursion_limit = "512"]

mod runtime;
mod util;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{
    parse_macro_input,
    AttributeArgs,
    ItemFn,
};
use util::set_fn_dummy;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn runtime(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as ItemFn);
    set_fn_dummy(&item);
    runtime::Attribute::from(attr_args).generate(item)
}
