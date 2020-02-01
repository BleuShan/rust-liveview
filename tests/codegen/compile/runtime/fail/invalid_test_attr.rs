#[allow(unused_imports)]
use async_std::task;
use rust_liveview::runtime;

fn main() {}

#[runtime::test(executor = "async_std")]
#[test]
async fn runtime_attribute_test_should_add_the_test_attribute() {}
