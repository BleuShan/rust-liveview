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

cfg_not_test! {
    pub use rust_liveview_codegen::runtime_entrypoint_main as main;
}

cfg_async_std_runtime! {
    pub use async_std::*;
}

pub use rust_liveview_codegen::runtime_entrypoint_test as test;

mod application;
pub use application::Application;

cfg_tls! {
    pub mod tls;
    pub use tls::TlsBuilder;
}
