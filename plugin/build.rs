use std::process::Command;

fn main() {
    let rustc_v = rustc_version::version()
        .expect("Unable to get rustc version")
        .to_string();
    let expected_v = "1.73.0".to_string();

    // Check for a minimum version
    if rustc_v != expected_v {
        println!(
            "cargo:warning=trying to compile with rustc {}, we should be using {}",
            rustc_v, expected_v,
        );
        std::process::exit(1);
    }
    println!("cargo:rustc-env=RUSTC_VERSION={:#?}", rustc_v,);

    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("unable to get git commit hash");
    let commit_hash = String::from_utf8(output.stdout).unwrap();
    let url = format!(
        "https://github.com/clockwork-xyz/clockwork/tree/{}/cli/Cargo.toml",
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
