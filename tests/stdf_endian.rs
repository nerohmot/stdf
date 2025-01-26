use std::process::Command;

#[test]
fn test_endian_le() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "stdf", "--", "endian", "-i", "tests/test_data/test.stdf"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(stdout.contains("LE"));
}

//TODO: Add test for BE