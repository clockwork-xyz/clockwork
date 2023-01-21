use clockwork_client::network::state::Snapshot;
use std::io::Write;

#[allow(deprecated)]
use {
    crate::{errors::CliError, parser::ProgramInfo},
    anyhow::Result,
    clockwork_client::{
        network::state::ConfigSettings,
        thread::state::{Thread, Trigger},
        Client,
    },
    regex::Regex,
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_instruction,
    },
    spl_associated_token_account::{create_associated_token_account, get_associated_token_address},
    spl_token::{
        instruction::{initialize_mint, mint_to},
        state::Mint,
    },
    std::process::{Child, Command},
};

pub fn start(
    client: &Client,
    clone_addresses: Vec<Pubkey>,
    network_url: Option<String>,
    program_infos: Vec<ProgramInfo>,
) -> Result<(), CliError> {
    check_test_validator_version();
    // Start the validator
    let validator_process =
        &mut start_test_validator(client, program_infos, network_url, clone_addresses)
            .map_err(|err| CliError::FailedLocalnet(err.to_string()))?;

    // Initialize Clockwork
    let mint_pubkey =
        mint_clockwork_token(client).map_err(|err| CliError::FailedTransaction(err.to_string()))?;
    super::initialize::initialize(client, mint_pubkey)?;
    register_worker(client).map_err(|err| CliError::FailedTransaction(err.to_string()))?;
    create_threads(client, mint_pubkey)
        .map_err(|err| CliError::FailedTransaction(err.to_string()))?;

    // Wait for process to be killed.
    _ = validator_process.wait();

    Ok(())
}

fn check_test_validator_version() {
    let validator_version = get_validator_version();
    let clockwork_version = env!("GEYSER_INTERFACE_VERSION");

    if validator_version != clockwork_version {
        let mut line = String::new();

        let err = format!(
            "Your Solana version and the Clockwork Engine's Solana version differs. \
            This behavior is undefined. \
            You have '{}' installed, but the Clockwork Engine requires {} \
            We recommend you to run `solana-install init {}`\nDo you want to continue anyway? \
            More info: https://github.com/clockwork-xyz/docs/blob/main/FAQ.md#clockwork-engine",
            validator_version, clockwork_version, clockwork_version
        );
        println!("⚠️  \x1b[93m{}️\x1b[0m", err);

        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap_or_default();
    }
}

fn get_validator_version() -> String {
    Command::new("solana-test-validator")
        .arg("--version")
        .output()
        .map_or("unknown".into(), |output| {
            let version = String::from_utf8_lossy(&output.stdout);
            let re = Regex::new(r"(\d\.\d{2}\.\d)").unwrap();
            let caps = re.captures(&version).unwrap();
            caps
                .get(1)
                .map_or("unknown (error parsing solana-validator version)", |m| {
                    m.as_str()
                })
                .into()
        })
}

fn mint_clockwork_token(client: &Client) -> Result<Pubkey> {
    // Calculate rent and pubkeys
    let mint_keypair = Keypair::new();
    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .unwrap_or(0);
    let token_account_pubkey =
        get_associated_token_address(&client.payer_pubkey(), &mint_keypair.pubkey());

    // Build ixs
    let ixs = vec![
        // Create mint account
        system_instruction::create_account(
            &client.payer_pubkey(),
            &mint_keypair.pubkey(),
            mint_rent,
            Mint::LEN as u64,
            &spl_token::ID,
        ),
        initialize_mint(
            &spl_token::ID,
            &mint_keypair.pubkey(),
            &client.payer_pubkey(),
            None,
            8,
        )
        .unwrap(),
        // Create associated token account
        #[allow(deprecated)]
        create_associated_token_account(
            &client.payer_pubkey(),
            &client.payer_pubkey(),
            &mint_keypair.pubkey(),
        ),
        // Mint 10 tokens to the local user
        mint_to(
            &spl_token::ID,
            &mint_keypair.pubkey(),
            &token_account_pubkey,
            &client.payer_pubkey(),
            &[&client.payer_pubkey()],
            1000000000,
        )
        .unwrap(),
    ];

    // Submit tx
    client.send_and_confirm(&ixs, &[client.payer(), &mint_keypair])?;

    Ok(mint_keypair.pubkey())
}

fn register_worker(client: &Client) -> Result<()> {
    // Create the worker
    let cfg = get_clockwork_config()?;
    let keypath = format!(
        "{}/lib/clockwork-worker-keypair.json",
        cfg["home"].as_str().unwrap()
    );
    let signatory = read_keypair_file(keypath).unwrap();
    client.airdrop(&signatory.pubkey(), LAMPORTS_PER_SOL)?;
    super::worker::create(client, signatory, true)?;

    // Delegate stake to the worker
    super::delegation::create(client, 0)?;
    super::delegation::deposit(client, 100000000, 0, 0)?;
    Ok(())
}

fn create_threads(client: &Client, mint_pubkey: Pubkey) -> Result<()> {
    // Create epoch thread.
    let epoch_thread_id = "clockwork.network.epoch";
    let epoch_thread_pubkey = Thread::pubkey(client.payer_pubkey(), epoch_thread_id.into());
    let ix_a = clockwork_client::thread::instruction::thread_create(
        client.payer_pubkey(),
        epoch_thread_id.into(),
        clockwork_client::network::instruction::registry_epoch_kickoff(
            Snapshot::pubkey(0),
            epoch_thread_pubkey,
        )
        .into(),
        client.payer_pubkey(),
        epoch_thread_pubkey,
        Trigger::Cron {
            schedule: "0 * * * * * *".into(),
            skippable: true,
        },
    );

    // Create hasher thread.
    let hasher_thread_id = "clockwork.network.hasher";
    let hasher_thread_pubkey = Thread::pubkey(client.payer_pubkey(), hasher_thread_id.into());
    let ix_b = clockwork_client::thread::instruction::thread_create(
        client.payer_pubkey(),
        hasher_thread_id.into(),
        clockwork_client::network::instruction::registry_nonce_hash(hasher_thread_pubkey).into(),
        client.payer_pubkey(),
        hasher_thread_pubkey,
        Trigger::Cron {
            schedule: "*/15 * * * * * *".into(),
            skippable: true,
        },
    );

    // Update config with thread pubkeys
    let ix_c = clockwork_client::network::instruction::config_update(
        client.payer_pubkey(),
        ConfigSettings {
            admin: client.payer_pubkey(),
            epoch_thread: epoch_thread_pubkey,
            hasher_thread: hasher_thread_pubkey,
            mint: mint_pubkey,
        },
    );

    client.send_and_confirm(&vec![ix_a, ix_b, ix_c], &[client.payer()])?;
    client.airdrop(&epoch_thread_pubkey, LAMPORTS_PER_SOL)?;
    client.airdrop(&hasher_thread_pubkey, LAMPORTS_PER_SOL)?;

    Ok(())
}

fn start_test_validator(
    client: &Client,
    program_infos: Vec<ProgramInfo>,
    network_url: Option<String>,
    clone_addresses: Vec<Pubkey>,
) -> Result<Child> {
    println!("Starting test validator");

    // Get Clockwork home path
    let cfg = get_clockwork_config()?;
    let home_dir = cfg["home"].as_str().unwrap();

    // TODO Build a custom plugin config
    let mut process = Command::new("solana-test-validator")
        .arg("-r")
        .bpf_program(home_dir, clockwork_client::network::ID, "network")
        .bpf_program(home_dir, clockwork_client::thread::ID, "thread")
        .bpf_program(home_dir, clockwork_client::webhook::ID, "webhook")
        .network_url(network_url)
        .clone_addresses(clone_addresses)
        .add_programs_with_path(program_infos)
        .geyser_plugin_config(home_dir)
        .spawn()
        .expect("Failed to start local test validator");

    // Wait for the validator to become healthy
    let ms_wait = 10_000;
    let mut count = 0;
    while count < ms_wait {
        match client.get_block_height() {
            Err(_err) => {
                std::thread::sleep(std::time::Duration::from_millis(1));
                count += 1;
            }
            Ok(slot) => {
                if slot > 0 {
                    println!("Got a slot: {}", slot);
                    break;
                }
            }
        }
    }
    if count == ms_wait {
        process.kill()?;
        std::process::exit(1);
    }

    // Wait 1 extra second for safety before submitting txs
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(process)
}

fn lib_path(home_dir: &str, filename: &str) -> String {
    format!("{}/lib/{}", home_dir, filename)
}

fn get_clockwork_config() -> Result<serde_yaml::Value> {
    let clockwork_config_path = dirs_next::home_dir()
        .map(|mut path| {
            path.extend(&[".config", "solana", "clockwork", "config.yml"]);
            path.to_str().unwrap().to_string()
        })
        .unwrap();
    let f = std::fs::File::open(clockwork_config_path)?;
    let clockwork_config: serde_yaml::Value = serde_yaml::from_reader(f)?;
    Ok(clockwork_config)
}

trait TestValidatorHelpers {
    fn add_programs_with_path(&mut self, program_infos: Vec<ProgramInfo>) -> &mut Command;
    fn bpf_program(
        &mut self,
        home_dir: &str,
        program_id: Pubkey,
        program_name: &str,
    ) -> &mut Command;
    fn geyser_plugin_config(&mut self, home_dir: &str) -> &mut Command;
    fn network_url(&mut self, url: Option<String>) -> &mut Command;
    fn clone_addresses(&mut self, clone_addresses: Vec<Pubkey>) -> &mut Command;
}

impl TestValidatorHelpers for Command {
    fn add_programs_with_path(&mut self, program_infos: Vec<ProgramInfo>) -> &mut Command {
        for program_info in program_infos {
            self.arg("--bpf-program")
                .arg(program_info.program_id.to_string())
                .arg(program_info.program_path);
        }

        self
    }
    fn bpf_program(
        &mut self,
        home_dir: &str,
        program_id: Pubkey,
        program_name: &str,
    ) -> &mut Command {
        let filename = format!("clockwork_{}_program.so", program_name);
        self.arg("--bpf-program")
            .arg(program_id.to_string())
            .arg(lib_path(home_dir, filename.as_str()))
    }

    fn geyser_plugin_config(&mut self, home_dir: &str) -> &mut Command {
        self.arg("--geyser-plugin-config")
            .arg(lib_path(home_dir, "geyser-plugin-config.json"))
    }

    fn network_url(&mut self, url: Option<String>) -> &mut Command {
        if let Some(url) = url {
            self.arg("--url").arg(url);
        }
        self
    }

    fn clone_addresses(&mut self, clone_addresses: Vec<Pubkey>) -> &mut Command {
        for clone_address in clone_addresses {
            self.arg("--clone").arg(clone_address.to_string());
        }
        self
    }
}
