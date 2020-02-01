#![allow(unused_imports)]

use async_std::task;
use fluid::prelude::*;
use rust_liveview::runtime;

fn main() {}

#[runtime::test(executor = "async_std", kind = "theory")]
async fn runtime_attribute_test_should_add_the_test_attribute() {}
