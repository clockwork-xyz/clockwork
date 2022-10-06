use std::str::FromStr;

use {
    crate::errors::CliError,
    clockwork_client::network::objects::{
        Config, Node, NodeSettings, Registry, Snapshot, SnapshotEntry,
    },
    clockwork_client::Client,
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
    },
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    },
    std::mem::size_of,
};

pub fn get(client: &Client, worker_pubkey: Pubkey) -> Result<(), CliError> {
    let memcmp = RpcFilterType::Memcmp(Memcmp {
        offset: 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<u64>(),
        bytes: MemcmpEncodedBytes::Base58(worker_pubkey.to_string()),
        encoding: None,
    });
    let config = RpcProgramAccountsConfig {
        filters: Some(vec![memcmp]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };
    let acc_infos = client
        .get_program_accounts_with_config(&clockwork_client::network::ID, config)
        .unwrap();
    for (pubkey, acc_info) in acc_infos {
        let node = Node::try_from(acc_info.data).unwrap();
        println!("Address: {:#?}\n{:#?}", pubkey, node);
    }
    Ok(())
}

pub fn update(
    client: &Client,
    node_pubkey: Pubkey,
    settings: NodeSettings,
) -> Result<(), CliError> {
    let ix = clockwork_client::network::instruction::node_update(
        client.payer_pubkey(),
        node_pubkey,
        settings,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}

pub fn register(client: &Client, worker: Keypair) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let owner = client.payer().clone();
    let registry_pubkey = Registry::pubkey();
    let registry_data = client
        .get_account_data(&registry_pubkey)
        .map_err(|_err| CliError::AccountNotFound(registry_pubkey.to_string()))?;
    let registry_data = Registry::try_from(registry_data)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    let node_pubkey = Node::pubkey(registry_data.node_count);

    let snapshot_pubkey = Snapshot::pubkey(registry_data.snapshot_count - 1);
    let entry_pubkey = SnapshotEntry::pubkey(snapshot_pubkey, registry_data.node_count);
    let ix = clockwork_client::network::instruction::node_register(
        owner.pubkey(),
        config_pubkey,
        entry_pubkey,
        config_data.mint,
        node_pubkey,
        registry_pubkey,
        snapshot_pubkey,
        worker.pubkey(),
    );
    client.send_and_confirm(&[ix], &[owner, &worker]).unwrap();
    Ok(())
}

pub fn stake(client: &Client, address: Pubkey, amount: u64) -> Result<(), CliError> {
    // Get config data
    let config_pubkey = Config::pubkey();
    let config_data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config_data = Config::try_from(config_data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Build ix
    let signer = client.payer();
    // let node_pubkey = Node::pubkey(worker);
    let ix = clockwork_client::network::instruction::node_stake(
        amount,
        config_pubkey,
        address,
        config_data.mint,
        signer.pubkey(),
    );

    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
