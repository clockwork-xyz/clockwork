use {
    crate::errors::CliError,
    crate::config::CliConfig,
    clockwork_utils::explorer::Explorer,

};

pub fn automation_url<T: std::fmt::Display>(automation: T, config: CliConfig) -> Result<(),
    CliError> {
    println!("automation: {}", explorer(config).automation_url(automation,
                                                 clockwork_client::automation::ID));
    Ok(())
}

fn explorer(config: CliConfig) -> Explorer {
   Explorer::from(config.json_rpc_url)
}
