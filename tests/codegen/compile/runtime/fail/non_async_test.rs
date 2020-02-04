use rust_liveview::runtime;

fn main() {}

#[runtime::test(async_std)]
fn runtime_attribute_test_should_add_the_test_attribute() {}
