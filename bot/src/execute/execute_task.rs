use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use {
    crate::{
        replicate::replicate_task,
        utils::{new_rpc_client, sign_and_submit},
    },
    cronos_sdk::account::*,
    std::thread,
};

pub fn execute_task(task: Pubkey, daemon: Pubkey, ix: InstructionData) {
    thread::spawn(move || {
        let client = new_rpc_client();
        let config = Config::pda().0;
        let fee = Fee::pda(daemon).0;

        // Add accounts to exec instruction
        let mut ix_exec =
            cronos_sdk::instruction::task_execute(config, daemon, fee, task, client.payer_pubkey());
        for acc in ix.accounts {
            match acc.is_writable {
                true => ix_exec.accounts.push(AccountMeta::new(acc.pubkey, false)),
                false => ix_exec
                    .accounts
                    .push(AccountMeta::new_readonly(acc.pubkey, false)),
            }
        }
        ix_exec
            .accounts
            .push(AccountMeta::new_readonly(ix.program_id, false));

        // Sign and submit
        let res = sign_and_submit(
            &client,
            &[ix_exec],
            format!("Executing task: {} {}", task, daemon).as_str(),
        );
        match res {
            Err(_err) => {
                // If exec failed, replicate the task data
                let data = client.get_account_data(&task).unwrap();
                let task_data = Task::try_from(data).unwrap();
                replicate_task(task, task_data)
            }
            _ => return,
        }
    });
}
