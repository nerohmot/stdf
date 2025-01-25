use std::process::Command;

#[test]
fn test_count_records() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "stdf", "--", "count", "-i", "tests/test_data/test.stdf"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    // Add assertions based on the expected output
    assert!(stdout.contains("Counting the records in the file"));
    // Add more specific assertions based on your program's expected output
}