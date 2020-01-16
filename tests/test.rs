use trybuild::TestCases;

#[test]
fn codegen_tests() {
    let test_cases = TestCases::new();
    test_cases.compile_fail("tests/codegen-ui/fail.rs");
}
