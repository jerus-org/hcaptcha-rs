use test_suite_cli::{assert_output, cargo_bin, load_expected, random_string, Cmd};

fn main() {
    let expected = load_expected();
    eprintln!("{expected}");

    let cmd = cargo_bin("hcaptcha-cli");

    let mut cmd = Cmd::new(&cmd);
    eprintln!("cmd: {:?}", cmd);
    let cmd = cmd
        .arg("--token")
        .arg("10000000-aaaa-bbbb-cccc-000000000001")
        .arg("--secret")
        .arg("0x0000000000000000000000000000000000000000");

    eprintln!("cmd to run: {:#?}", cmd);

    let output_res = cmd.output();

    eprintln!("output result: {:#?}", output_res);

    let output = output_res.expect("failed to spawn");

    assert_output(output, &expected);
}
