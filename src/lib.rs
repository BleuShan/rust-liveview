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

pub use rust_liveview_codegen::runtime;
pub use rust_liveview_common as common;
pub use rust_liveview_rpc as rpc;
pub use rust_liveview_view as view;
