use test_suite_cli::{assert_output, cargo_bin, load_expected, Cmd};

fn main() {
    let expected = load_expected();

    let cmd = cargo_bin("hcaptcha-cli");
    let mut cmd = Cmd::new(&cmd);

    let output = cmd.arg("--help").output().expect("failed to spawn");

    assert_output(output, &expected);
}
