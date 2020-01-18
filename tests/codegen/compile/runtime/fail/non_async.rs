#[allow(unused_imports)]
use async_std::task;
use rust_liveview::runtime;

#[runtime(executor_entrypoint = "task::block_on")]
fn main() {}
