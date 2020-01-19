use rust_liveview::view::Element;

fn main() {}

#[derive(Debug, Element)]
struct EnumElement {
    #[element(self_closing)]
    test: bool,
}
