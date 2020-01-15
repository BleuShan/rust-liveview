use fluid::prelude::*;

#[fact]
fn it_should_fail() {
    (2 + 2).should().not().be_equal_to(4);
}
