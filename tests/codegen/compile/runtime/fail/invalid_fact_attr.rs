#![allow(unused_imports)]

use fluid::prelude::*;
use rust_liveview::runtime::{
    self,
    task,
};

fn main() {}

#[runtime::test(async_std)]
#[fact]
async fn runtime_attribute_test_should_add_the_test_attribute() {}
