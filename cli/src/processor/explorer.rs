use {
    crate::errors::CliError,
    clockwork_utils::explorer::Explorer,
};

pub fn thread_url<T: std::fmt::Display>(thread: T) -> Result<(), CliError> {
    println!("thread: {}", explorer().thread_url(thread,
                                                 clockwork_client::thread::ID));
    Ok(())
}

fn explorer() -> Explorer {
    Explorer::devnet()
}