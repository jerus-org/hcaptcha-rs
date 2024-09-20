use std::{io, path::Path, process::Output};
use std::{path::PathBuf, process::Command};

#[cfg(target_os = "linux")]
pub fn cargo_bin(name: &str) -> std::path::PathBuf {
    let bin_dir = bin_dir();
    bin_dir.join(name)
}

#[cfg(target_os = "wasi")]
const WASM_SUFFIX: &str = "wasm";

#[cfg(target_os = "wasi")]
pub fn cargo_bin(name: &str) -> std::path::PathBuf {
    let file_name = format!("{}{}", name, WASM_SUFFIX);
    let bin_dir = bin_dir();
    bin_dir.join(file_name)
}

fn bin_dir() -> PathBuf {
    PathBuf::new().join("bin")
}

pub fn assert_output(output: std::process::Output, expected: &str) {
    // eprintln!("output:{:#?}", &output);

    if !output.stdout.is_empty() {
        let o = String::from_utf8_lossy(&output.stdout);
        eprintln!("{o}");

        assert_eq!(o, expected);
    };

    if !output.stderr.is_empty() {
        let e = String::from_utf8_lossy(&output.stderr);
        eprintln!("{e}");

        assert_eq!(e, expected);
    };
}

#[derive(Debug)]
pub struct Cmd {
    pub inner_cmd: Command,
}
use std::env;

impl Cmd {
    pub fn new(binary: &Path) -> Self {
        eprintln!("cmd:{:?}", &binary);

        if env::var("WASI").is_ok() {
            let inner_cmd = Command::new("wasmtime");
            let mut cmd = Cmd { inner_cmd };
            cmd.inner_cmd
                .arg(binary.to_str().expect("failed to convert path to str"));
            cmd
        } else {
            let inner_cmd = Command::new(binary);
            Cmd { inner_cmd }
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.inner_cmd.arg(arg);
        self
    }

    pub fn output(&mut self) -> io::Result<Output> {
        self.inner_cmd.output()
    }
}
