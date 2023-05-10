use {
    crate::deps::ToTagVersion,
    clap::crate_version,
    solana_sdk::commitment_config::CommitmentConfig,
    std::{
        env,
        fs,
        path::PathBuf,
        time::Duration,
    },
};

pub const DEFAULT_RPC_TIMEOUT_SECONDS: Duration = Duration::from_secs(30);
pub const DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS: Duration = Duration::from_secs(5);
pub const RELAYER_URL: &str = "http://localhost:8000/";
pub const CLOCKWORK_RELEASE_BASE_URL: &str =
    "https://github.com/clockwork-xyz/clockwork/releases/download";
pub const CLOCKWORK_DEPS: &[&str] = &[
    "clockwork_network_program.so",
    "clockwork_thread_program.so",
    "clockwork_webhook_program.so",
    "libclockwork_plugin.so",
];
pub const SOLANA_RELEASE_BASE_URL: &str = "https://github.com/solana-labs/solana/releases/download";
pub const SOLANA_DEPS: &[&str] = &["solana-test-validator"];

/// The combination of solana config file and our own config file
#[derive(Debug, PartialEq)]
pub struct CliConfig {
    pub json_rpc_url: String,
    pub websocket_url: String,
    pub relayer_url: String,
    pub keypair_path: String,
    pub rpc_timeout: Duration,
    pub commitment: CommitmentConfig,
    pub confirm_transaction_initial_timeout: Duration,

    pub active_version: String,
    pub dev: bool,
}

impl CliConfig {
    pub fn load() -> Self {
        let solana_config_file = solana_cli_config::CONFIG_FILE.as_ref().unwrap().as_str();
        let solana_config = solana_cli_config::Config::load(solana_config_file).unwrap();
        CliConfig {
            json_rpc_url: solana_config.json_rpc_url,
            websocket_url: solana_config.websocket_url,
            relayer_url: RELAYER_URL.to_owned(),
            keypair_path: solana_config.keypair_path,
            rpc_timeout: DEFAULT_RPC_TIMEOUT_SECONDS,
            commitment: CommitmentConfig::confirmed(),
            confirm_transaction_initial_timeout: DEFAULT_CONFIRM_TX_TIMEOUT_SECONDS,
            active_version: crate_version!().to_owned().to_tag_version(),
            dev: false,
        }
    }

    pub fn default_home() -> PathBuf {
        dirs_next::home_dir()
            .map(|mut path| {
                path.extend([".config", "clockwork"]);
                path
            })
            .unwrap()
    }

    pub fn default_runtime_dir() -> PathBuf {
        let mut path = Self::default_home();
        path.extend(["localnet", "runtime_deps"]);
        path
    }

    pub fn active_runtime_dir(&self) -> PathBuf {
        Self::default_runtime_dir().join(&self.active_version)
    }

    pub fn target_dir(&self) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.extend(["..", "target"]);
        fs::canonicalize(path).unwrap()
    }

    pub fn active_runtime(&self, filename: &str) -> String {
        if self.dev == true {
            if filename.contains("solana") {
                self.active_runtime_dir().join(filename).to_string()
            } else if filename.contains("program") {
                self.target_dir().join("deploy").join(filename).to_string()
            } else {
                self.target_dir().join("debug").join(filename).to_string()
            }
        } else {
            self.active_runtime_dir().join(filename).to_string()
        }
    }

    /// This assumes the path for the signatory keypair created by solana-test-validator
    /// is test-ledger/validator-keypair.json
    pub fn signatory(&self) -> String {
        env::current_dir()
            .map(|mut path| {
                path.extend(["test-ledger", "validator-keypair.json"]);
                path
            })
            .expect(&format!(
                "Unable to find location of validator-keypair.json"
            ))
            .to_string()
    }

    pub fn geyser_config(&self) -> String {
        self.active_runtime("geyser-plugin-config.json")
    }

    pub fn geyser_lib(&self) -> String {
        if self.dev == true && env::consts::OS.to_lowercase().contains("mac") {
            self.active_runtime("libclockwork_plugin.dylib")
        } else {
            // in the release process, we always rename dylib to so anyway
            self.active_runtime("libclockwork_plugin.so")
        }
    }
}

pub trait PathToString {
    fn to_string(&self) -> String;
}

impl PathToString for PathBuf {
    fn to_string(&self) -> String {
        self.clone().into_os_string().into_string().unwrap()
    }
}

// Clockwork Deps Helpers
impl CliConfig {
    // #[tokio::main]
    fn detect_target_triplet() -> String {
        let output = std::process::Command::new("cargo")
            .arg("-vV")
            .output()
            .expect("failed to execute process");

        let host_prefix = "host:";
        String::from_utf8(output.stdout)
            .expect("Unable to get output from cargo -vV")
            .split('\n')
            .find(|line| line.trim_start().to_lowercase().starts_with(&host_prefix))
            .map(|line| line.trim_start_matches(&host_prefix).trim())
            .expect("Unable to detect target 'host' from cargo -vV")
            .to_owned()
    }

    pub fn clockwork_release_url(tag: &str) -> String {
        format!(
            "{}/{}/{}",
            CLOCKWORK_RELEASE_BASE_URL,
            tag,
            &Self::clockwork_release_archive()
        )
    }

    pub fn clockwork_release_archive() -> String {
        let target_triplet = Self::detect_target_triplet();
        format!("clockwork-geyser-plugin-release-{}.tar.bz2", target_triplet)
    }

    pub fn solana_release_url(tag: &str) -> String {
        format!(
            "{}/{}/{}",
            SOLANA_RELEASE_BASE_URL,
            tag,
            &Self::solana_release_archive()
        )
    }

    pub fn solana_release_archive() -> String {
        let target_triplet = Self::detect_target_triplet();
        format!("solana-release-{}.tar.bz2", target_triplet)
    }
}
