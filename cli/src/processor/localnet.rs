#[allow(deprecated)]
use {
    crate::{
        client::Client,
        config::CliConfig,
        deps,
        errors::CliError,
        parser::ProgramInfo,
        print::print_style,
        print_status
    },
    anyhow::{
        Context,
        Result,
    },
    anchor_lang::{
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas
    },
    clap::crate_version,
    clockwork_network_program::state::{Config, ConfigSettings, Registry},
    clockwork_thread_program::state::{Thread, Trigger},
    solana_sdk::{
        native_token::LAMPORTS_PER_SOL,
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{
            read_keypair_file,
            Keypair,
            Signer,
        },
        system_instruction,
    },
    spl_associated_token_account::{
        create_associated_token_account,
        get_associated_token_address,
    },
    spl_token::{
        instruction::{
            initialize_mint,
            mint_to,
        },
        state::Mint,
    },
    std::fs,
    std::process::{
        Child,
        Command,
    },
};

pub fn start(
    config: &mut CliConfig,
    client: &Client,
    clone_addresses: Vec<Pubkey>,
    network_url: Option<String>,
    program_infos: Vec<ProgramInfo>,
    force_init: bool,
    solana_archive: Option<String>,
    clockwork_archive: Option<String>,
    dev: bool,
) -> Result<(), CliError> {
    config.dev = dev;
    deps::download_deps(
        &CliConfig::default_runtime_dir(),
        force_init,
        solana_archive,
        clockwork_archive,
        dev,
    )
    .map_err(|err| CliError::FailedLocalnet(err.to_string()))?;

    // Create Geyser Plugin Config file
    create_geyser_plugin_config(config).map_err(|err| CliError::FailedLocalnet(err.to_string()))?;

    // Start the validator
    let validator_process =
        &mut start_test_validator(config, client, program_infos, network_url, clone_addresses)
            .map_err(|err| CliError::FailedLocalnet(err.to_string()))?;

    // Initialize Clockwork
    let mint_pubkey =
        mint_clockwork_token(client).map_err(|err| CliError::FailedTransaction(err.to_string()))?;
    super::initialize::initialize(client, mint_pubkey)
        .map_err(|err| CliError::FailedTransaction(err.to_string()))?;
    register_worker(client, config).map_err(|err| CliError::FailedTransaction(err.to_string()))?;
    create_threads(client, mint_pubkey)
        .map_err(|err| CliError::FailedTransaction(err.to_string()))?;

    // Wait for process to be killed.
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
    client
        .send_and_confirm(&ixs, &[client.payer(), &mint_keypair])
        .context("mint_clockwork_token failed")?;

    Ok(mint_keypair.pubkey())
}

fn register_worker(client: &Client, config: &CliConfig) -> Result<()> {
    // Create the worker
    let signatory = read_keypair_file(&config.signatory()).map_err(|err| {
        CliError::FailedLocalnet(format!(
            "Unable to read keypair {}: {}",
            &config.signatory(),
            err
        ))
    })?;

    client
        .airdrop(&signatory.pubkey(), LAMPORTS_PER_SOL)
        .context("airdrop to signatory failed")?;
    super::worker::create(client, signatory, true).context("worker::create failed")?;

    // Delegate stake to the worker
    super::delegation::create(client, 0).context("delegation::create failed")?;
    super::delegation::deposit(client, 100000000, 0, 0).context("delegation::deposit failed")?;
    Ok(())
}

fn create_threads(client: &Client, mint_pubkey: Pubkey) -> Result<()> {
    // Create epoch thread.
    let epoch_thread_id = "clockwork.network.epoch";
    let epoch_thread_pubkey = Thread::pubkey(client.payer_pubkey(), epoch_thread_id.into());
    let ix_a1 = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::DistributeFeesJob {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::DistributeFeesJob {}.data(),
    };
    let ix_a2 = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::ProcessUnstakesJob {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::ProcessUnstakesJob {}.data(),
    };
    let ix_a3 = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::StakeDelegationsJob {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::StakeDelegationsJob {}.data(),
    };
    let ix_a4 = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::TakeSnapshotJob {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::TakeSnapshotJob {}.data(),
    };
    let ix_a5 = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::EpochCutover {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::IncrementEpoch {}.data(),
    };
    let ix_a6 = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::DeleteSnapshotJob {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::DeleteSnapshotJob {}.data(),
    };
    let ix_a = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            system_program: system_program::ID,
            thread: epoch_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadCreate {
            amount: LAMPORTS_PER_SOL,
            id: epoch_thread_id.into(),
            instructions: vec![
                ix_a1.into(),
                ix_a2.into(),
                ix_a3.into(),
                ix_a4.into(),
                ix_a5.into(),
                ix_a6.into(),
            ],
            trigger: Trigger::Cron {
                schedule: "0 * * * * * *".into(),
                skippable: true,
            },
        }
        .data(),
    };

    // Create hasher thread.
    let hasher_thread_id = "clockwork.network.hasher";
    let hasher_thread_pubkey = Thread::pubkey(client.payer_pubkey(), hasher_thread_id.into());
    let registry_hash_ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::RegistryNonceHash {
            config: Config::pubkey(),
            registry: Registry::pubkey(),
            thread: hasher_thread_pubkey,  
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::RegistryNonceHash {}.data(),
    };
    let ix_b = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            system_program: system_program::ID,
            thread: hasher_thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadCreate {
            amount: LAMPORTS_PER_SOL,
            id: hasher_thread_id.into(),
            instructions: vec![
                registry_hash_ix.into(),
            ],
            trigger: Trigger::Cron {
                schedule: "*/15 * * * * * *".into(),
                skippable: true,
            },
        }
        .data(),
    };

    // Update config with thread pubkeys
    let settings = ConfigSettings {
        admin: client.payer_pubkey(),
        epoch_thread: epoch_thread_pubkey,
        hasher_thread: hasher_thread_pubkey,
        mint: mint_pubkey,
    };
    let ix_c = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::ConfigUpdate {
            admin: client.payer_pubkey(),
            config: Config::pubkey()
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::ConfigUpdate { settings }.data(),
    };

    client
        .send_and_confirm(&vec![ix_a], &[client.payer()])
        .context(format!(
            "Failed to create thread: {} or update config",
            epoch_thread_id,
        ))?;
    client
        .send_and_confirm(&vec![ix_b, ix_c], &[client.payer()])
        .context(format!("Failed to create thread: {}", hasher_thread_id))?;

    Ok(())
}

fn create_geyser_plugin_config(config: &CliConfig) -> Result<()> {
    let geyser_config = clockwork_plugin_utils::PluginConfig {
        keypath: Some(config.signatory().to_owned()),
        libpath: Some(config.geyser_lib().to_owned()),
        ..Default::default()
    };

    let content = serde_json::to_string_pretty(&geyser_config)
        .context("Unable to serialize PluginConfig to json")?;
    let path = &config.geyser_config();
    fs::write(&path, content).context(format!("Unable to serialize PluginConfig to {}", path))?;
    Ok(())
}

fn start_test_validator(
    config: &CliConfig,
    client: &Client,
    program_infos: Vec<ProgramInfo>,
    network_url: Option<String>,
    clone_addresses: Vec<Pubkey>,
) -> Result<Child> {
    let path = config.active_runtime("solana-test-validator").to_owned();
    let cmd = &mut Command::new(path);
    cmd.arg("-r")
        .bpf_program(config, clockwork_network_program::ID, "network")
        .bpf_program(config, clockwork_thread_program::ID, "thread")
        .bpf_program(config, clockwork_webhook_program::ID, "webhook")
        .network_url(network_url)
        .clone_addresses(clone_addresses)
        .add_programs_with_path(program_infos)
        .geyser_plugin_config(config);

    let mut process = cmd
        .spawn()
        .context(format!("solana-test-validator command: {:#?}", cmd))?;
print_status!("Running", "Clockwork Validator {}\n", crate_version!());

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

trait TestValidatorHelpers {
    fn add_programs_with_path(&mut self, program_infos: Vec<ProgramInfo>) -> &mut Command;
    fn bpf_program(
        &mut self,
        config: &CliConfig,
        program_id: Pubkey,
        program_name: &str,
    ) -> &mut Command;
    fn geyser_plugin_config(&mut self, config: &CliConfig) -> &mut Command;
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
        config: &CliConfig,
        program_id: Pubkey,
        program_name: &str,
    ) -> &mut Command {
        let filename = format!("clockwork_{}_program.so", program_name);
        self.arg("--bpf-program")
            .arg(program_id.to_string())
            .arg(config.active_runtime(filename.as_str()).to_owned())
    }

    fn geyser_plugin_config(&mut self, config: &CliConfig) -> &mut Command {
        self.arg("--geyser-plugin-config")
            .arg(config.geyser_config().to_owned())
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
