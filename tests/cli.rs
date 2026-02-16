use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

// Satisfies: R-01-01 — --interface flag is accepted; verified by its presence in --help output
#[test]
fn cli_help_flag() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--interface"));
}

// General CLI robustness: unknown flags are rejected (clap default; no specific requirement)
#[test]
fn cli_unknown_flag() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--unknown-flag")
        .assert()
        .failure();
}

// Version output correctness (no specific requirement)
#[test]
fn cli_version_flag() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

// Satisfies: R-01-03 — no --interface flag prints available interfaces; lo must appear
#[test]
fn cli_no_args_lists_interfaces() {
    cargo_bin_cmd!("packet_sniffer")
        .assert()
        .success()
        .stdout(predicate::str::contains("lo"));
}

// Satisfies: R-01-02 — valid --interface flag starts capture on that interface
#[test]
fn cli_valid_interface_prints_name() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--interface")
        .arg("lo")
        .assert()
        .success()
        .stdout(predicate::str::contains("interface: lo"));
}

// Satisfies: R-01-09 — unknown interface exits non-zero with the interface name in the error
#[test]
fn cli_unknown_interface_exits_nonzero_with_name() {
    cargo_bin_cmd!("packet_sniffer")
        .arg("--interface")
        .arg("fake0")
        .assert()
        .failure()
        .stderr(predicate::str::contains("fake0"));
}
