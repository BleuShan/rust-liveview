#![allow(clippy::new_ret_no_self)]

use rust_liveview_common::Builder;

#[derive(Builder, Debug)]
#[builder(setter(strip_option, into), build_fn(skip), pattern = "owned")]
pub struct Application {
    test: usize,
}

impl Application {
    pub fn new() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }
}

impl ApplicationBuilder {
    pub async fn build(self) -> Application {
        Application { test: 0 }
    }
}
