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

use rust_liveview_common::cfg_not_test;

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod private;

mod application;
pub mod tls;

cfg_not_test! {
    pub use rust_liveview_codegen::runtime_entrypoint_main as main;
}

pub use rust_liveview_codegen::runtime_entrypoint_test as test;

pub use application::Application;
pub use tls::TlsBuilder;
