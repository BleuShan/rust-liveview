#![warn(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]
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

mod macros;
pub use derive_more::*;
pub use http;
pub use inflector::{
    Inflector,
    InflectorNumbers,
};
pub use language_tags;
pub use mime;
pub use thiserror;
