use log::LevelFilter;
use log4rs_test_utils::test_logging;

#[test]
fn run_without_any_args() {
    test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

    let expected = String::from(
        r#"error: the following required arguments were not provided:
  --token <TOKEN>
  --secret <SECRET>

Usage: hcaptcha-cli --token <TOKEN> --secret <SECRET>

For more information, try '--help'.
"#,
    );

    let cmd = snapbox::cmd::cargo_bin!("hcaptcha-cli");

    log::debug!("cmd:{:?}", &cmd);

    let output = snapbox::cmd::Command::new(cmd).output();

    if let Ok(output) = output {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::debug!("output:{:?}", &stderr);

        assert_eq!(stderr, expected);
    };
}

#[test]
fn run_request_help_info_short() {
    test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

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

    let cmd = snapbox::cmd::cargo_bin!("hcaptcha-cli");

    log::debug!("cmd:{:?}", &cmd);

    let output = snapbox::cmd::Command::new(cmd).arg("-h").output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::debug!("output:{:?}", &stdout);

        assert_eq!(stdout, expected);
    };
}

#[test]
fn run_request_help_info_long() {
    test_logging::init_logging_once_for(vec![], LevelFilter::Debug, None);

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

    let cmd = snapbox::cmd::cargo_bin!("hcaptcha-cli");

    log::debug!("cmd:{:?}", &cmd);

    let output = snapbox::cmd::Command::new(cmd).arg("--help").output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        log::debug!("output:{:?}", &stdout);

        assert_eq!(stdout, expected);
    };
}
