use rust_liveview::view::prelude::*;

fn main() {}

#[derive(Debug, Element)]
struct EnumElement {
    #[element(self_closing)]
    test: bool,
}
