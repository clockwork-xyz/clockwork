use solana_sdk::native_token::LAMPORTS_PER_SOL;

use {
    crate::{
        cli::CliError, parser::JsonInstructionData, utils::new_client, utils::sign_and_submit,
    },
    chrono::{prelude::*, Duration},
    cronos_sdk::scheduler::state::{Daemon, Fee, Task},
    serde_json::json,
    solana_client_helpers::Client,
    solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer},
    std::sync::Arc,
};

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let client = new_client();
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let num_tasks_serial = count - num_tasks_parallel;

    let mut owners: Vec<Keypair> = vec![];

    // Create daemons
    for _ in 0..(num_tasks_parallel + 1) {
        let owner = Keypair::new();
        let daemon_pda = Daemon::pda(owner.pubkey());
        let daemon_addr = daemon_pda.0;
        let fee_pda = Fee::pda(daemon_addr);
        let ix =
            cronos_sdk::scheduler::instruction::daemon_new(daemon_pda, fee_pda, owner.pubkey());
        client.airdrop(&owner.pubkey(), LAMPORTS_PER_SOL).unwrap();
        sign_and_submit(&client, &[ix], &owner);
        owners.push(owner);
    }

    // Schedule parallel tasks
    for i in 0..num_tasks_parallel {
        let owner = owners.get(i as usize).unwrap();
        schedule_memo_task(&client, owner, recurrence);
    }

    // Schedule serial tasks
    let owner = owners.last().unwrap();
    for _ in 0..num_tasks_serial {
        schedule_memo_task(&client, owner, recurrence);
    }

    // TODO Loop and monitor for task execs
    // TODO Print performance results to stdout

    Ok(())
}

fn build_memo_ix(daemon_pubkey: &Pubkey) -> Instruction {
    let hello_world_memo = json!({
      "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
      "accounts": [
        {
          "pubkey": daemon_pubkey.to_string(),
          "is_signer": true,
          "is_writable": false
        }
      ],
      "data": [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
    });
    Instruction::try_from(
        &serde_json::from_value::<JsonInstructionData>(hello_world_memo)
            .expect("JSON was not well-formatted"),
    )
    .unwrap()
}

fn schedule_memo_task(client: &Arc<Client>, owner: &Keypair, recurrence: u32) {
    let daemon_pubkey = Daemon::pda(owner.pubkey()).0;
    let memo_ix = build_memo_ix(&daemon_pubkey);
    let daemon_data = client
        .get_account_data(&daemon_pubkey)
        .map_err(|_err| CliError::AccountNotFound(daemon_pubkey.to_string()))
        .unwrap();
    let daemon = Daemon::try_from(daemon_data)
        .map_err(|_err| CliError::AccountDataNotParsable(daemon_pubkey.to_string()))
        .unwrap();
    let task_pda = Task::pda(daemon_pubkey, daemon.task_count);
    let now: DateTime<Utc> = Utc::now();
    let next_minute = now + Duration::minutes(1);
    let schedule = format!(
        "0-{} {} {} {} {} {} {}",
        recurrence,
        next_minute.minute(),
        next_minute.hour(),
        next_minute.day(),
        next_minute.month(),
        next_minute.weekday(),
        next_minute.year()
    );
    let create_task_ix = cronos_sdk::scheduler::instruction::task_new(
        task_pda,
        daemon_pubkey,
        owner.pubkey(),
        vec![memo_ix],
        schedule,
    );
    sign_and_submit(&client, &[create_task_ix], owner);
}
