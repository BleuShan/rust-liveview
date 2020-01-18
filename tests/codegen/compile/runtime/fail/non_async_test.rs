#[allow(unused_imports)]
use async_std::task;
use rust_liveview::runtime;

fn main() {}

#[runtime(executor_entrypoint = "task::block_on", test)]
fn runtime_attribute_should_add_the_test_attribute() {}
