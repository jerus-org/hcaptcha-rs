use test_suite_cli::{assert_output, cargo_bin, Cmd};

fn main() {
    let expected = String::from(
        r#"Usage: hcaptcha-cli [OPTIONS] --token <TOKEN> --secret <SECRET>

Options:
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -t, --token <TOKEN>    The captcha token proivded by the client for validation
  -k, --key <KEY>        The sitekey for hcapthca validation
  -s, --secret <SECRET>  The secret key for hcaptcha validation
  -i, --ip <IP>          The ip address of the system generating the request
  -h, --help             Print help
  -V, --version          Print version
"#,
    );

    let cmd = cargo_bin("hcaptcha-cli");
    let mut cmd = Cmd::new(&cmd);

    let output = cmd.arg("-h").output().expect("failed to spawn");

    assert_output(output, &expected);
}
