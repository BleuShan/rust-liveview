mod runtime;

#[test]
fn compile_tests() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/codegen/compile/**/fail/**/*.rs");
    test_cases.pass("tests//codegen/compile/**/pass/**/*.rs");
}
