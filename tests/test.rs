use trybuild::TestCases;

#[test]
fn compile_tests() {
    let test_cases = TestCases::new();
    test_cases.compile_fail("tests/compile/**/fail/**/*.rs");
    test_cases.pass("tests/compile/**/pass/**/*.rs");
}
