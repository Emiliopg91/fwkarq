use std::process::Command;

use crate::shell::Shell;

#[test]
fn test_01_run_which_ok() {
    assert!(Shell::run_command(Command::new("which").arg("ls"), true).is_ok())
}

#[test]
fn test_02_run_which_err() {
    assert!(Shell::run_command(Command::new("whicsh").arg("ls"), true).is_err())
}

#[test]
fn test_03_run_program_ok() {
    assert!(Shell::command("which").unwrap().arg("ls").run(true).is_ok())
}

#[test]
fn test_04_run_program_err() {
    assert!(
        Shell::command("which")
            .unwrap()
            .arg("lss")
            .run(true)
            .is_err()
    );
}
