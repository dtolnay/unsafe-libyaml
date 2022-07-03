use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn test(id: &str) {
    let dir = Path::new("tests")
        .join("data")
        .join("yaml-test-suite")
        .join(id);

    let output = Command::new(env!("CARGO_BIN_EXE_run_emitter_test_suite"))
        .arg(dir.join("test.event"))
        .stdin(Stdio::null())
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    eprint!("{}", stderr);

    let out = if dir.join("out.yaml").exists() {
        dir.join("out.yaml")
    } else {
        dir.join("in.yaml")
    };
    let expected = fs::read_to_string(out).unwrap();
    pretty_assertions::assert_str_eq!(expected, stdout);
    assert!(output.status.success());
}

unsafe_libyaml_test_suite::test_emitter!();
