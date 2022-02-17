use dotenv::dotenv;
use solana_client_helpers::ClientResult;

mod env;
mod execute;
mod replicate;
mod utils;

use {execute::*, replicate::*};

fn main() -> ClientResult<()> {
    // Load env file
    dotenv().ok();

    // Replicate Cronos tasks to Postgres
    replicate_cronos_tasks();

    // Process pending tasks when Solana blocktime updates
    process_tasks();

    println!("❌ Exiting");
    Ok(())
}
