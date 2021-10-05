#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/captcha_not_identified.rs");
    t.compile_fail("tests/ui/not_a_struct.rs");
    t.pass("tests/ui/simple.rs");
}
