use test_suite_cli::{assert_output, cargo_bin, Cmd};

fn main() {
    let expected = String::from(
        r#"error: the following required arguments were not provided:
  --token <TOKEN>
  --secret <SECRET>

Usage: hcaptcha-cli --token <TOKEN> --secret <SECRET>

For more information, try '--help'.
"#,
    );

    let cmd = cargo_bin("hcaptcha-cli");
    let mut cmd = Cmd::new(&cmd);

    let output = cmd.output().expect("failed to spawn");

    assert_output(output, &expected);
}
