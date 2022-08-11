use {
    crate::errors::CliError,
    anyhow::Result,
    clockwork_client::Client,
    solana_sdk::pubkey::Pubkey,
    std::process::{Child, Command},
};

pub fn start(client: &Client) -> Result<(), CliError> {
    // Start the validator
    let validator_process =
        &mut start_test_validator(client).map_err(|_| CliError::FailedLocalnet)?;

    // Wait for process to be killed
    _ = validator_process.wait();

    Ok(())
}

fn start_test_validator(client: &Client) -> Result<Child> {
    // Get Clockwork home path
    let clockwork_config_path = dirs_next::home_dir()
        .map(|mut path| {
            path.extend(&[".config", "solana", "clockwork", "config.yml"]);
            path.to_str().unwrap().to_string()
        })
        .unwrap();
    let f = std::fs::File::open(clockwork_config_path)?;
    let clockwork_config: serde_yaml::Value = serde_yaml::from_reader(f)?;
    let home_dir = clockwork_config["home"].as_str().unwrap();

    // TODO Build a custom plugin config
    let mut process = Command::new("solana-test-validator")
        .arg("-r")
        .bpf_program(home_dir, clockwork_client::health::ID, "health")
        .bpf_program(home_dir, clockwork_client::http::ID, "http")
        .bpf_program(home_dir, clockwork_client::network::ID, "network")
        .bpf_program(home_dir, clockwork_client::pool::ID, "pool")
        .bpf_program(home_dir, clockwork_client::scheduler::ID, "scheduler")
        .geyser_plugin_config(home_dir)
        .spawn()
        .expect("Failed to start local test validator");

    // Wait for the validator to become healthy
    let ms_wait = 10000;
    let mut count = 0;
    while count < ms_wait {
        let r = client.get_latest_blockhash();
        if r.is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
        count += 1;
    }
    if count == ms_wait {
        process.kill()?;
        std::process::exit(1);
    }

    // Wait 1 extra second for safety before submitting txs
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(process)
}

fn lib_path(home_dir: &str, filename: &str) -> String {
    format!("{}/lib/{}", home_dir, filename)
}

trait TestValidatorHelpers {
    fn bpf_program(
        &mut self,
        home_dir: &str,
        program_id: Pubkey,
        program_name: &str,
    ) -> &mut Command;
    fn geyser_plugin_config(&mut self, home_dir: &str) -> &mut Command;
}

impl TestValidatorHelpers for Command {
    fn bpf_program(
        &mut self,
        home_dir: &str,
        program_id: Pubkey,
        program_name: &str,
    ) -> &mut Command {
        let filename = format!("clockwork_{}.so", program_name);
        self.arg("--bpf-program")
            .arg(program_id.to_string())
            .arg(lib_path(home_dir, filename.as_str()))
    }

    fn geyser_plugin_config(&mut self, home_dir: &str) -> &mut Command {
        self.arg("--geyser-plugin-config")
            .arg(lib_path(home_dir, "geyser-plugin-config.json"))
    }
}
