use cronos_sdk::account::*;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};

use std::thread;
use std::sync::{Arc, RwLock};

use crate::store::{TaskStore, MutableTaskStore};
use crate::utils::{new_rpc_client, sign_and_submit};

pub fn execute_task(store: Arc<RwLock<TaskStore>>, key: Pubkey, task: Task) {
    thread::spawn(move || {
        let client = new_rpc_client();
        let config = Config::pda().0;
        let fee = Fee::pda(task.daemon).0;

        // Add accounts to exec instruction
        let mut ix_exec =
            cronos_sdk::instruction::task_execute(config, task.daemon, fee, key, client.payer_pubkey());
        for acc in task.ix.accounts {
            match acc.is_writable {
                true => ix_exec.accounts.push(AccountMeta::new(acc.pubkey, false)),
                false => ix_exec
                    .accounts
                    .push(AccountMeta::new_readonly(acc.pubkey, false)),
            }
        }
        ix_exec
            .accounts
            .push(AccountMeta::new_readonly(task.ix.program_id, false));

        // Sign and submit
        let res = sign_and_submit(
            &client,
            &[ix_exec],
            format!("Executing task: {} {}", key, task.daemon).as_str(),
        );
        match res {
            Err(_err) => {
                // If exec failed, replicate the task data
                let data = client.get_account_data(&key).unwrap();
                let task = Task::try_from(data).unwrap();
                let mut w_store = store.write().unwrap();
                w_store.insert(key, task)
            }
            _ => return,
        }
    });
}
