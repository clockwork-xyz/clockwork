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
    // let health_program_path = format!("{}{}-{}", build_dir, version, "libclockwork_health.so");
    // let http_program_path = format!("{}{}-{}", build_dir, version, "libclockwork_http.so");
    // let network_program_path = format!("{}{}-{}", build_dir, version, "libclockwork_network.so");
    // let pool_program_path = format!("{}{}-{}", build_dir, version, "libclockwork_pool.so");
    // let scheduler_program_path =
    //     format!("{}{}-{}", build_dir, version, "libclockwork_scheduler.so");

    // TODO Build a custom plugin config
    let mut process = Command::new("solana-test-validator")
        .arg("-r")
        // .add_bpf_program(clockwork_client::health::ID, health_program_path)
        // .add_bpf_program(clockwork_client::http::ID, http_program_path)
        // .add_bpf_program(clockwork_client::network::ID, network_program_path)
        // .add_bpf_program(clockwork_client::pool::ID, pool_program_path)
        // .add_bpf_program(clockwork_client::scheduler::ID, scheduler_program_path)
        // .add_geyser_plugin()
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

trait TestValidatorHelpers {
    fn add_bpf_program(&mut self, program_id: Pubkey, program_path: String) -> &mut Command;
    // fn add_geyser_plugin(&mut self) -> &mut Command;
}

impl TestValidatorHelpers for Command {
    fn add_bpf_program(&mut self, program_id: Pubkey, program_path: String) -> &mut Command {
        self.arg("--bpf-program")
            .arg(program_id.to_string())
            .arg(program_path)
    }

    // fn add_geyser_plugin(&mut self) -> &mut Command {
    //     self.arg("--geyser-plugin-config")
    //         .arg(CLOCKWORK_GEYSER_PLUGIN_CONFIG_PATH)
    // }
}
