use cargo_toml::{Dependency, Manifest};
use regex::Regex;

fn main() {
    let geyser_interface_version = get_geyser_interface_version();
    println!(
        "cargo:rustc-env=GEYSER_INTERFACE_VERSION={}",
        geyser_interface_version
    );
}

fn get_geyser_interface_version() -> String {
    let plugin_manifest = Manifest::from_path("../plugin/Cargo.toml").unwrap();
    let plugin_interface = plugin_manifest
        .dependencies
        .get("solana-geyser-plugin-interface")
        .unwrap();

    let semver = match plugin_interface {
        Dependency::Simple(version) => version.into(),
        Dependency::Detailed(detail) => detail.version.as_ref().unwrap().into(),
        _ => "unknown (error parsing Plugin's Cargo.toml)".to_string(),
    };

    let re = Regex::new(r"(\d\.\d{2}\.\d)").unwrap();
    re.captures(&semver)
        .unwrap()
        .get(1)
        .map_or("unknown (error parsing solana-geyser-plugin-interface version)", |m| {
            m.as_str()
        })
        .into()
}
