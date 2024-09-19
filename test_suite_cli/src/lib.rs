use std::process::Command;
use std::{io, path::Path, process::Output};

#[cfg(target_os = "linux")]
pub fn cargo_bin(name: &str) -> std::path::PathBuf {
    // let file_name = format!("{}{}", name, std::env::consts::EXE_SUFFIX);
    let target_dir = target_dir();
    // target_dir.join(file_name)
    target_dir.join(name)
}

#[cfg(target_os = "wasi")]
const WASM_SUFFIX: &str = "wasm";

#[cfg(target_os = "wasi")]
pub fn cargo_bin(name: &str) -> std::path::PathBuf {
    let file_name = format!("{}{}", name, std::env::consts::WASM_SUFFIX);
    let target_dir = target_dir();
    target_dir.join(file_name)
}

// Adapted from
// https://github.com/rust-lang/cargo/blob/485670b3983b52289a2f353d589c57fae2f60f82/tests/testsuite/support/mod.rs#L507
fn target_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .ok()
        .map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
        .unwrap()
}

pub fn assert_output(output: std::process::Output, expected: &str) {
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

#[derive(Debug)]
pub struct Cmd {
    pub inner_cmd: Command,
}

impl Cmd {
    #[cfg(target_os = "linux")]
    pub fn new(binary: &Path) -> Self {
        eprintln!("cmd:{:?}", &binary);
        let inner_cmd = Command::new(binary);
        Cmd { inner_cmd }
    }

    #[cfg(target_os = "wasi")]
    pub fn new(binary: &Path) -> Self {
        eprintln!("cmd:{:?}", &binary);
        let inner_cmd = Command::new("wasmtime");
        let mut cmd = Cmd { inner_cmd };
        cmd.inner_cmd
            .arg(binary.to_str().expect("failed to convert path to str"));
        cmd
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.inner_cmd.arg(arg);
        self
    }

    pub fn output(&mut self) -> io::Result<Output> {
        self.inner_cmd.output()
    }
}
