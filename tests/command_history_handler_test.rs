use temp_env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use nebula_env::command_history_handler::{get_history_path, store_command, HISTORY_FILE};

#[test]
fn test_get_history_path_success() {
    let temp_dir = TempDir::new().expect("Failed to get temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    temp_env::with_var("HOME", Some(temp_path), || {
        let path = get_history_path().expect("Expected a valid path");

        assert_eq!(path, PathBuf::from(format!("{}/{}", temp_path, HISTORY_FILE)));
    });
}

#[test]
fn test_store_command() {
    let temp_dir = TempDir::new().expect("Failed to get temp dir");
    let temp_file_path = temp_dir.path().join(HISTORY_FILE);

    let command = "echo Hello World";

    store_command(command, temp_file_path.clone())
        .expect("Must store the command in the specified path");

    let contents = fs::read_to_string(&temp_file_path)
        .expect("Must read the stored file");

    assert!(contents.contains(command));
}

#[test]
fn test_store_command_appends_commands() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_file_path = temp_dir.path().join(HISTORY_FILE);

    let command1 = "echo First Command";
    let command2 = "echo Second Command";

    store_command(command1, temp_file_path.clone()).expect("Must store first command");
    store_command(command2, temp_file_path.clone()).expect("Must store second command");

    let contents = fs::read_to_string(&temp_file_path)
        .expect("Must read the command without error");

    assert!(contents.contains(command1));
    assert!(contents.contains(command2));
}

