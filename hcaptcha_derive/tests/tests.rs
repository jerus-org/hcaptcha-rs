#[test]
pub fn expand_pass() {
    macrotest::expand("tests/expand/*.rs");
    // macrotest::expand_without_refresh("tests/expand/*.rs");
}

#[test]
pub fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
