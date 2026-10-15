//! End-to-end tests that run the compiled binary.

use std::process::Command;

#[test]
fn binary_prints_greeting() {
    let output = Command::new(env!("CARGO_BIN_EXE_fountain-stage"))
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
}
