#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/captcha_not_identified.rs");
    t.compile_fail("tests/ui/not_a_struct.rs");
    t.pass("tests/ui/simple.rs");
    t.pass("tests/ui/with_remoteip.rs");
    t.pass("tests/ui/with_sitekey.rs");
    t.pass("tests/ui/complete_success_response.rs");
    t.pass("tests/ui/complete_error_response.rs");
}
