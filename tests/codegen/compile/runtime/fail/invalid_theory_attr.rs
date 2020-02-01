#![allow(unused_imports)]

use async_std::task;
use fluid::prelude::*;
use rust_liveview::runtime;

fn main() {}

#[runtime::test(executor = "async_std")]
#[theory]
#[case(1)]
async fn runtime_attribute_test_should_add_the_test_attribute(a: usize) {
    assert_eq!(1, a)
}
