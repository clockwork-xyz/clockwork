use cargo_toml::{Dependency, Manifest};
use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    let url = format!(
        "https://github.com/clockwork-xyz/clockwork/tree/{}/plugin/Cargo.toml",
        commit_hash
    );
    println!("cargo:rustc-env=SPEC={}", url);

    let geyser_interface_version = get_geyser_interface_version();
    println!(
        "cargo:rustc-env=GEYSER_INTERFACE_VERSION={}",
        geyser_interface_version
    );
}

fn get_geyser_interface_version<'a>() -> String {
    let plugin_manifest = Manifest::from_path("./Cargo.toml").unwrap();
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
