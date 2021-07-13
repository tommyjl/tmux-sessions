use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_no_args() {
    Command::cargo_bin("tsesh").unwrap().assert().failure();
}

#[test]
fn cli_version_long() {
    Command::cargo_bin("tsesh")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(format!("tsesh {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_version_short() {
    Command::cargo_bin("tsesh")
        .unwrap()
        .arg("-V")
        .assert()
        .success()
        .stdout(format!("tsesh {}\n", env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_start_zero_args() {
    Command::cargo_bin("tsesh")
        .unwrap()
        .args(&["start", "--config", "example.toml"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains(
                "error: The following required arguments were not provided:\n",
            )
            .and(predicate::str::contains("<names>")),
        );
}

#[test]
fn cli_restart_zero_args() {
    Command::cargo_bin("tsesh")
        .unwrap()
        .args(&["restart", "--config", "example.toml"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains(
                "error: The following required arguments were not provided:\n",
            )
            .and(predicate::str::contains("<names>")),
        );
}

#[test]
fn cli_stop_zero_args() {
    Command::cargo_bin("tsesh")
        .unwrap()
        .args(&["stop"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains(
                "error: The following required arguments were not provided:\n",
            )
            .and(predicate::str::contains("<names>")),
        );
}
