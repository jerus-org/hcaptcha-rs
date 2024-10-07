#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compilefail/*.rs");
    t.pass("tests/ui/pass/test_*.rs");
}
