use {
    chrono::{prelude::*, Duration},
    serde_json::json,
    solana_sdk::instruction::Instruction,
};

use crate::{error::TestError, parser::*, utils::new_client, utils::sign_and_submit};

pub fn test(count: u32, time: u32, percent: f32) -> Result<(), TestError> {
    let daemons_p = count as f32 * percent; // number of daemons that are parallelized
    let mut remaining_tasks = count; // count of remaining tasks to schedule serialized task in single daemon

    // creating x parallellized daaemons
    for _i in 0..daemons_p as i32 {
        // reinitialize client to be able to create new daemons each loop
        let client = new_client();
        let owner = client.payer_pubkey();

        let daemon_pda = cronos_sdk::scheduler::state::Daemon::pda(owner);
        let daemon_addr = daemon_pda.0;
        let fee_pda = cronos_sdk::scheduler::state::Fee::pda(daemon_addr);

        let ix = cronos_sdk::scheduler::instruction::daemon_new(daemon_pda, fee_pda, owner);

        // sign and submit tx to create new daemon
        sign_and_submit(&client, &[ix]);

        let memo = json!({
          "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
          "accounts": [
            {
              "pubkey": daemon_addr.to_string(),
              "is_signer": true,
              "is_writable": false
            }
          ],
          "data": [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
        });

        let ix_json = serde_json::from_value::<JsonInstructionData>(memo)
            .expect("JSON was not well-formatted");

        let ix = Instruction::try_from(&ix_json).unwrap();

        let data = client
            .get_account_data(&daemon_addr)
            .map_err(|_err| TestError::AccountNotFound(daemon_addr.to_string()))
            .unwrap();

        let daemon_data = cronos_sdk::scheduler::state::Daemon::try_from(data)
            .map_err(|_err| TestError::AccountDataNotParsable(daemon_addr.to_string()))
            .unwrap();

        let task_pda = cronos_sdk::scheduler::state::Task::pda(daemon_addr, daemon_data.task_count);

        let now: DateTime<Utc> = Utc::now(); // now real time

        println!("{} {}", now.date(), now.time());

        let buffer = now + Duration::minutes(2 as i64); // buffer to schedule all tasks

        let schedule = format!(
            "0-{} {} {} {} {} {} {}",
            time,
            buffer.minute(),
            buffer.hour(),
            buffer.day(),
            buffer.month(),
            buffer.weekday(),
            buffer.year()
        );

        // let expression = "0-20 30 14 2 May Mon 2022";
        // let schedule = Schedule::from_str(&expression).unwrap();
        // println!("Upcoming fire times:");
        // for datetime in schedule.upcoming(Utc) {
        //     println!("-> {}", datetime);
        // }

        let task_ix = cronos_sdk::scheduler::instruction::task_new(
            task_pda,
            daemon_addr,
            owner,
            vec![ix],
            schedule,
        );

        sign_and_submit(&client, &[task_ix]);
        remaining_tasks -= 1;
    }

    // if the parallelization variable isn't 100% then serialize the remaining number of tasks into a single daemon
    if percent < 1.0 {
        let client = new_client();
        let owner = client.payer_pubkey();

        let daemon_pda = cronos_sdk::scheduler::state::Daemon::pda(owner);
        let daemon_addr = daemon_pda.0;
        let fee_pda = cronos_sdk::scheduler::state::Fee::pda(daemon_addr);

        let ix = cronos_sdk::scheduler::instruction::daemon_new(daemon_pda, fee_pda, owner);

        sign_and_submit(&client, &[ix]);

        let memo = json!({
          "program_id": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
          "accounts": [
            {
              "pubkey": daemon_addr.to_string(),
              "is_signer": true,
              "is_writable": false
            }
          ],
          "data": [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
        });

        let ix_json = serde_json::from_value::<JsonInstructionData>(memo)
            .expect("JSON was not well-formatted");

        for _i in 0..remaining_tasks {
            let ix = Instruction::try_from(&ix_json).unwrap();

            let data = client
                .get_account_data(&daemon_addr)
                .map_err(|_err| TestError::AccountNotFound(daemon_addr.to_string()))
                .unwrap();

            let daemon_data = cronos_sdk::scheduler::state::Daemon::try_from(data)
                .map_err(|_err| TestError::AccountDataNotParsable(daemon_addr.to_string()))
                .unwrap();

            let task_pda =
                cronos_sdk::scheduler::state::Task::pda(daemon_addr, daemon_data.task_count);

            let now: DateTime<Utc> = Utc::now(); // now real time

            // println!("{} {}", now.date(), now.time());

            let buffer = now + Duration::minutes(2 as i64); // buffer to schedule all tasks

            let schedule = format!(
                "0-{} {} {} {} {} {} {}",
                time,
                buffer.minute(),
                buffer.hour(),
                buffer.day(),
                buffer.month(),
                buffer.weekday(),
                buffer.year()
            );

            let task_ix = cronos_sdk::scheduler::instruction::task_new(
                task_pda,
                daemon_addr,
                owner,
                vec![ix],
                schedule,
            );

            sign_and_submit(&client, &[task_ix]);
        }
    }

    Ok(())
}
