use test_suite_cli::{assert_output, cargo_bin, load_expected, Cmd};

fn main() {
    //     let expected = String::from(
    //         r#"Usage: hcaptcha-cli [OPTIONS] --token <TOKEN> --secret <SECRET>

    // Options:
    //   -v, --verbose...       Increase logging verbosity
    //   -q, --quiet...         Decrease logging verbosity
    //   -t, --token <TOKEN>    The captcha token proivded by the client for validation
    //   -k, --key <KEY>        The sitekey for hcapthca validation
    //   -s, --secret <SECRET>  The secret key for hcaptcha validation
    //   -i, --ip <IP>          The ip address of the system generating the request
    //   -h, --help             Print help
    //   -V, --version          Print version
    // "#,
    //     );

    let expected = load_expected();
    eprintln!("{expected}");

    let cmd = cargo_bin("hcaptcha-cli");

    let mut cmd = Cmd::new(&cmd);
    println!("cmd: {:?}", cmd);

    let output_res = cmd.arg("-h").output();

    println!("output result: {:#?}", output_res);

    let output = output_res.expect("failed to spawn");

    assert_output(output, &expected);
}
