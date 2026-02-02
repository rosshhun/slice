//! Integration tests for the `slice` CLI.
//! Run with: cargo test

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_head_basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Line 1\nLine 2\nLine 3\nLine 4")?;

    let mut cmd = Command::cargo_bin("slice")?;
    cmd.arg("--head").arg("2").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Line 1\nLine 2"));

    Ok(())
}

#[test]
fn test_tail_basic() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "Line 1\nLine 2\nLine 3\nLine 4")?;

    let mut cmd = Command::cargo_bin("slice")?;
    cmd.arg("--tail").arg("2").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Line 3\nLine 4"));

    Ok(())
}

#[test]
fn test_lines_range() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A\nB\nC\nD\nE")?;

    let mut cmd = Command::cargo_bin("slice")?;
    cmd.arg("--lines").arg("2-4").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("B\nC\nD"));

    Ok(())
}

#[test]
fn test_help_succeeds() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("slice")?;
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Slice text by lines"));

    Ok(())
}

#[test]
fn test_no_args_prints_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("slice")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage"));

    Ok(())
}
