use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn cli_help_flag() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--interface"));
}

#[test]
fn cli_unknown_flag() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--unknown-flag")
        .assert()
        .failure();
}

#[test]
fn cli_version_flag() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}
