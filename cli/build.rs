use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("unable to get git commit hash");
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    let url = format!(
        "https://github.com/clockwork-xyz/clockwork/tree/{}/plugin/Cargo.toml",
        commit_hash
    );
    println!("cargo:rustc-env=SPEC={}", url);

    let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
    let geyser_interface_version = metadata
        .packages
        .iter()
        .find(|p| p.name == "solana-geyser-plugin-interface")
        .expect("Unable to parse solana-geyser-plugin-interface version using cargo metadata")
        .version
        .to_string();
    println!(
        "cargo:rustc-env=GEYSER_INTERFACE_VERSION={}",
        geyser_interface_version
    );
}
