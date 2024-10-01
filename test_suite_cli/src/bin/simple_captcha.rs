use test_suite_cli::{assert_output, cargo_bin, load_expected, Cmd};

fn main() {
    let expected = load_expected();
    eprintln!("{expected}");

    let cmd = cargo_bin("hcaptcha-cli");

    let mut cmd = Cmd::new(&cmd);
    println!("cmd: {:?}", cmd);
    let output_res = cmd
        .arg("--key")
        .arg("10000000-ffff-ffff-ffff-000000000001")
        .arg("--secret")
        .arg("0x0000000000000000000000000000000000000000")
        .output();

    println!("output result: {:#?}", output_res);

    let output = output_res.expect("failed to spawn");

    assert_output(output, &expected);
}
