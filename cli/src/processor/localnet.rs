use clockwork_client::pool::state::Pool;

#[allow(deprecated)]
use {
    crate::{errors::CliError, parser::ProgramInfo},
    anyhow::Result,
    clockwork_client::{network::state::Node, Client},
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

pub fn start(client: &Client, program_infos: Vec<ProgramInfo>) -> Result<(), CliError> {
    // Start the validator
    let validator_process = &mut start_test_validator(client, program_infos)
        .map_err(|err| CliError::FailedLocalnet(err.to_string()))?;

    // Initialize Clockwork
    let mint_pubkey =
        mint_clockwork_token(client).map_err(|err| CliError::FailedTransaction(err.to_string()))?;
    super::initialize::initialize(client, mint_pubkey)?;
    register_worker(client).map_err(|err| CliError::FailedTransaction(err.to_string()))?;

    // Wait for process to be killed
    _ = validator_process.wait();

    Ok(())
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
        // Mint 1000 tokens to the local user
        mint_to(
            &spl_token::ID,
            &mint_keypair.pubkey(),
            &token_account_pubkey,
            &client.payer_pubkey(),
            &[&client.payer_pubkey()],
            100000000000,
        )
        .unwrap(),
    ];

    // Submit tx
    client.send_and_confirm(&ixs, &[client.payer(), &mint_keypair])?;

    Ok(mint_keypair.pubkey())
}

fn register_worker(client: &Client) -> Result<()> {
    let cfg = get_clockwork_config()?;
    let keypath = format!(
        "{}/lib/clockwork-worker-keypair.json",
        cfg["home"].as_str().unwrap()
    );
    let worker_keypair = read_keypair_file(keypath).unwrap();
    client.airdrop(&worker_keypair.pubkey(), LAMPORTS_PER_SOL)?;
    super::node::register(client, worker_keypair)?;
    let node_pubkey = Node::pubkey(0);
    let pool_pubkey = Pool::pubkey("crank".into());
    super::node::stake(client, node_pubkey, 100)?;
    super::node::add_pool(client, node_pubkey, pool_pubkey)?;
    Ok(())
}

fn start_test_validator(client: &Client, program_infos: Vec<ProgramInfo>) -> Result<Child> {
    // Get Clockwork home path
    let cfg = get_clockwork_config()?;
    let home_dir = cfg["home"].as_str().unwrap();

    // TODO Build a custom plugin config
    let mut process = Command::new("solana-test-validator")
        .arg("-r")
        .bpf_program(home_dir, clockwork_client::crank::ID, "crank")
        .bpf_program(home_dir, clockwork_client::http::ID, "http")
        .bpf_program(home_dir, clockwork_client::network::ID, "network")
        .bpf_program(home_dir, clockwork_client::pool::ID, "pool")
        .add_programs_with_path(program_infos)
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
