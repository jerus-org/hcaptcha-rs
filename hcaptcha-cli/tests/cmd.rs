use std::process::Command;

/// The absolute path to a binary target's executable.
///
/// The `bin_target_name` is the name of the binary
/// target, exactly as-is.
///
/// **NOTE:** This is only set when building an integration test or benchmark.
///
/// ## Example
///
/// ```rust,no_run
/// #[test]
/// fn cli_tests() {
///     trycmd::TestCases::new()
///         .default_bin_path(trycmd::cargo_bin!("bin-fixture"))
///         .case("tests/cmd/*.trycmd");
/// }
/// ```
#[macro_export]
macro_rules! cargo_bin {
    ($bin_target_name:expr) => {
        ::std::path::Path::new(env!(concat!("CARGO_BIN_EXE_", $bin_target_name)))
    };
}

fn assert_output(output: std::process::Output, expected: &str) {
    eprintln!("output:{:?}", &output);

    if !output.stdout.is_empty() {
        let o = String::from_utf8_lossy(&output.stdout);
        eprintln!("output:{:?}", o);

        assert_eq!(o, expected);
    };

    if !output.stderr.is_empty() {
        let e = String::from_utf8_lossy(&output.stderr);
        eprintln!("error:{:?}", e);

        assert_eq!(e, expected);
    };
}

#[test]
fn run_without_any_args() {
    let expected = String::from(
        r#"error: the following required arguments were not provided:
  --token <TOKEN>
  --secret <SECRET>

Usage: hcaptcha-cli --token <TOKEN> --secret <SECRET>

For more information, try '--help'.
"#,
    );

    let cmd = cargo_bin!("hcaptcha-cli");

    eprintln!("cmd:{:?}", &cmd);

    let output = Command::new(cmd).output().expect("failed to spawn");

    assert_output(output, &expected);
}

#[test]
fn run_request_help_info_short() {
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

    let cmd = cargo_bin!("hcaptcha-cli");

    eprintln!("cmd:{:?}", &cmd);

    let output = Command::new(cmd)
        .arg("-h")
        .output()
        .expect("failed to spawn");

    assert_output(output, &expected);
}

#[test]
fn run_request_help_info_long() {
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

    let cmd = cargo_bin!("hcaptcha-cli");

    eprintln!("cmd:{:?}", &cmd);

    let output = Command::new(cmd)
        .arg("--help")
        .output()
        .expect("failed to spawn");

    assert_output(output, &expected);
}
