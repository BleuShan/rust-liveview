#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![feature(
    associated_type_defaults,
    box_patterns,
    box_syntax,
    error_iter,
    stdsimd,
    never_type,
    trait_alias,
    type_alias_impl_trait,
    try_blocks,
    try_trait
)]

#[macro_use]
mod macros;

cfg_test! {
    mod tests;
}
