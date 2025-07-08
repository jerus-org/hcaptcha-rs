use std::{io, path::Path, process::Output};
use std::{path::PathBuf, process::Command};

const WASM_SUFFIX: &str = ".wasm";

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

pub fn cargo_bin(name: &str) -> std::path::PathBuf {
    let bin = if env::var("WASI").is_ok() {
        let file_name = format!("{name}{WASM_SUFFIX}");
        let bin_dir = bin_dir();
        bin_dir.join(file_name)
    } else {
        let bin_dir = bin_dir();
        bin_dir.join(name)
    };
    eprintln!("cli binary:{:?}", &bin);

    bin
}

fn bin_dir() -> PathBuf {
    PathBuf::new().join("bin")
}

pub fn load_expected() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    eprintln!("path:{:?}", &path);
    let name = std::env::current_exe().unwrap();
    let name = name.file_stem().unwrap().to_str().unwrap();
    eprintln!("name:{:?}", &name);
    let wasm = if env::var("WASI").is_ok() {
        ".wasm"
    } else {
        ""
    };
    path.push(format!("testdata/{name}{wasm}.txt").as_str());
    eprintln!("expected path:{:?}", &path);
    std::fs::read_to_string(path).expect("failed to read file")
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
