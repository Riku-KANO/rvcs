use std::fs;
use std::process::Command;
use tempfile::TempDir;

fn run_rvcs(args: &[&str], dir: &std::path::Path) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rvcs"));
    cmd.current_dir(dir);
    cmd.args(args);
    cmd.output().expect("failed to execute process")
}

#[test]
fn test_init() {
    let temp_dir = TempDir::new().unwrap();
    let output = run_rvcs(&["init"], temp_dir.path());
    assert!(output.status.success());
    assert!(temp_dir.path().join(".rvcs").exists());
    assert!(temp_dir.path().join(".rvcs/objects").exists());
    assert!(temp_dir.path().join(".rvcs/index").exists());
}

#[test]
fn test_add() {
    let temp_dir = TempDir::new().unwrap();
    run_rvcs(&["init"], temp_dir.path());

    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello world").unwrap();

    let output = run_rvcs(&["add", "test.txt"], temp_dir.path());
    assert!(output.status.success());

    // Check object exists
    // sha256("hello world") = b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    let objects_dir = temp_dir.path().join(".rvcs/objects");
    let entries: Vec<_> = fs::read_dir(objects_dir)
        .unwrap()
        .map(|res| res.unwrap().file_name())
        .collect();
    assert!(!entries.is_empty());
}

#[test]
fn test_commit() {
    let temp_dir = TempDir::new().unwrap();
    run_rvcs(&["init"], temp_dir.path());

    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "hello world").unwrap();

    run_rvcs(&["add", "test.txt"], temp_dir.path());

    let output = run_rvcs(&["commit", "-m", "initial commit"], temp_dir.path());
    assert!(output.status.success());

    let output_str = String::from_utf8(output.stdout).unwrap();
    assert!(output_str.contains("initial commit"));

    // Check HEAD updated
    let head_content = fs::read_to_string(temp_dir.path().join(".rvcs/HEAD")).unwrap();
    assert!(head_content.starts_with("ref: refs/heads/main"));

    let ref_content = fs::read_to_string(temp_dir.path().join(".rvcs/refs/heads/main")).unwrap();
    assert!(!ref_content.is_empty());
}
