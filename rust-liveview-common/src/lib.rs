//! Common types and utilities

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
    const_if_match,
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
pub use bytes;
pub use derive_builder::Builder;
pub use derive_more::*;
pub use derive_new::new;
pub use http;
pub use inflector::{
    Inflector,
    InflectorNumbers,
};
pub use language_tags;
pub use once_cell;
pub use regex;
pub use thiserror;

pub mod mime;
pub mod pin;
pub mod strum;
