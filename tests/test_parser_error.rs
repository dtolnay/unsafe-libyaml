use std::path::Path;
use std::process::{Command, Stdio};

fn test(id: &str) {
    let dir = Path::new("tests")
        .join("data")
        .join("yaml-test-suite")
        .join(id);

    let output = Command::new(env!("CARGO_BIN_EXE_run_parser_test_suite"))
        .arg(dir.join("in.yaml"))
        .stdin(Stdio::null())
        .output()
        .unwrap();

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprint!("{}", stdout);
        eprint!("{}", stderr);
        panic!("expected parse to fail");
    }
}

unsafe_libyaml_test_suite::test_parser_error!();
