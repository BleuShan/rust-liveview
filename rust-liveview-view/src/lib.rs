//! This crate provides the necessary infrastructure to render a view hiearchy into an
//! html document.

#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
#![deny(missing_docs)]
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
pub mod element;
pub mod elements;
pub mod error;
pub mod prelude;
pub mod render;
pub mod result;
pub mod text;
