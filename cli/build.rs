use cargo_toml::{Dependency, Manifest};
use regex::Regex;
use std::process::Command;

fn main() {
    let validator_version = get_validator_version();
    let geyser_interface_version = get_geyser_interface_version();

    println!("cargo:rustc-env=VALIDATOR_VERSION={}", validator_version);
    println!(
        "cargo:rustc-env=GEYSER_INTERFACE_VERSION={}",
        geyser_interface_version
    );
}

fn get_validator_version() -> String {
    let output = Command::new("solana-test-validator")
        .arg("--version")
        .output()
        .unwrap();
    let version = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"(\d{1}\.\d{2}\.\d{1})").unwrap();
    let caps = re.captures(&version).unwrap();
    caps.get(1)
        .map_or(
            "unknown (error parsing solana-test-validator version)",
            |m| m.as_str(),
        )
        .into()
}

fn get_geyser_interface_version() -> String {
    let plugin_manifest = Manifest::from_path("../plugin/Cargo.toml").unwrap();
    let plugin_interface = plugin_manifest
        .dependencies
        .get("solana-geyser-plugin-interface")
        .unwrap();

    match plugin_interface {
        Dependency::Simple(version) => version.into(),
        Dependency::Detailed(detail) => detail.version.as_ref().unwrap().into(),
        _ => "unknown (error parsing Cargo.toml)".to_string(),
    }
}
