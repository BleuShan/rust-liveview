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

pub use fluid::prelude::*;

mod codegen;
mod view;
