use {
    crate::{
        cli::CliError, parser::JsonInstructionData, utils::new_client, utils::sign_and_submit,
    },
    chrono::{prelude::*, Duration},
    cronos_sdk::scheduler::state::{Action, Fee, Queue, Task},
    serde_json::json,
    solana_client_helpers::Client,
    solana_sdk::{
        instruction::Instruction, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
        signature::Keypair, signer::Signer,
    },
    std::sync::Arc,
};

pub fn run(count: u32, parallelism: f32, recurrence: u32) -> Result<(), CliError> {
    // Setup test
    let client = new_client();
    let num_tasks_parallel = (count as f32 * parallelism) as u32;
    let num_tasks_serial = count - num_tasks_parallel;

    let mut owners: Vec<Keypair> = vec![];

    // Create queues
    for _ in 0..(num_tasks_parallel + 1) {
        let owner = Keypair::new();
        let queue_pda = Queue::pda(owner.pubkey());
        let queue_addr = queue_pda.0;
        let fee_pda = Fee::pda(queue_addr);
        let ix = cronos_sdk::scheduler::instruction::queue_new(fee_pda, owner.pubkey(), queue_pda);
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

fn build_memo_ix(queue_pubkey: &Pubkey) -> Instruction {
    let hello_world_memo = json!({
      "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
      "accounts": [
        {
          "pubkey": queue_pubkey.to_string(),
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
    let now: DateTime<Utc> = Utc::now();
    let next_minute = now + Duration::minutes(1);
    let queue_pubkey = Queue::pda(owner.pubkey()).0;
    let queue_data = client
        .get_account_data(&queue_pubkey)
        .map_err(|_err| CliError::AccountNotFound(queue_pubkey.to_string()))
        .unwrap();
    let queue = Queue::try_from(queue_data)
        .map_err(|_err| CliError::AccountDataNotParsable(queue_pubkey.to_string()))
        .unwrap();
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
    let task_pda = Task::pda(queue_pubkey, queue.task_count);
    let create_task_ix = cronos_sdk::scheduler::instruction::task_new(
        owner.pubkey(),
        queue_pubkey,
        schedule,
        task_pda,
    );
    let action_pda = Action::pda(task_pda.0, 0);
    let memo_ix = build_memo_ix(&queue_pubkey);
    let create_action_ix = cronos_sdk::scheduler::instruction::action_new(
        action_pda,
        vec![memo_ix],
        owner.pubkey(),
        queue_pubkey,
        task_pda.0,
    );
    sign_and_submit(&client, &[create_task_ix, create_action_ix], owner);
}
