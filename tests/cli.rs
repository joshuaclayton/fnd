use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_assert_output_contains_full_path() {
    assert!(run_fnd(vec![]).contains("./src/cli.rs"));
    assert!(run_fnd(vec![]).contains("./src/main.rs"));
    assert!(run_fnd(vec![]).contains("./src/flags.rs"));
}

#[test]
fn test_string_match() {
    assert!(run_fnd(vec!["cli"]).contains("./src/cli.rs"));
    assert!(!run_fnd(vec!["cli"]).contains("./src/main.rs"));
}

#[test]
fn test_case_insensitive_string_match() {
    assert!(run_fnd(vec!["CLI", "-i"]).contains("./src/cli.rs"));
    assert!(run_fnd(vec!["readme", "-i"]).contains("./README.md"));
}

#[test]
fn test_regex_interpretation() {
    assert!(run_fnd(vec!["src/\\S{2}i\\.rs$", "-r"]).contains("./src/cli.rs"));
    assert!(!run_fnd(vec!["src/\\S{2}I\\.RS$", "-r"]).contains("./src/cli.rs"));
}

#[test]
fn test_case_insensitive_regex_interpretation() {
    assert!(run_fnd(vec!["src/\\S{2}i\\.rs$", "-r"]).contains("./src/cli.rs"));
    assert!(run_fnd(vec!["src/\\S{2}I\\.RS$", "-r", "-i"]).contains("./src/cli.rs"));
}

#[test]
fn test_hidden() {
    assert!(run_fnd(vec!["-h"]).contains("./.git/object"));
    assert!(!run_fnd(vec!["-h"]).contains("./target/debug"));
}

#[test]
fn test_all() {
    assert!(run_fnd(vec!["-a"]).contains("./.git/object"));
    assert!(run_fnd(vec!["-a"]).contains("./target/debug"));
}

fn run_fnd(args: Vec<&str>) -> String {
    let mut cmd = Command::cargo_bin("fnd").expect("fnd should exist");
    for arg in args {
        cmd.arg(arg);
    }

    std::str::from_utf8(&cmd.output().expect("stdout should exist").stdout)
        .expect("failed to parse output")
        .to_string()
}
