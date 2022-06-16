use cronos_client::network::state::Registry;

use {crate::cli::CliError, cronos_client::Client};

pub fn get(client: &Client) -> Result<(), CliError> {
    let registry_pubkey = cronos_client::network::state::Registry::pda().0;
    let registry = client
        .get::<Registry>(&registry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;
    println!("{:#?}", registry);

    // let mut node_pubkeys = vec![];
    // for i in 0..registry.node_count.min(10) {
    //     let node_pubkey = cronos_client::network::state::Node::pda(delegate);
    //     node_pubkeys.push(node_pubkey);
    // }
    // println!("Nodes: {:#?}", node_pubkeys);

    Ok(())
}
