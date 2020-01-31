#[allow(unused_imports)]
use async_std::task;
use rust_liveview::runtime;

#[runtime::main(executor = "async_std")]
fn main() {}
