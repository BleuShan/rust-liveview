#![allow(unused_imports)]

use fluid::prelude::*;
use rust_liveview::runtime;

fn main() {}

#[runtime::test(async_std, theory)]
async fn runtime_attribute_test_should_add_the_test_attribute(a: usize, b: usize) {}
