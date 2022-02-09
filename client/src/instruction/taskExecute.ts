import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { buildRemainingAccounts } from "./utils";
import { Cronos } from "../idl";
import { Account } from "../account";

export type TaskExecuteArgs = {
  task: PublicKey;
  worker: PublicKey;
};

export class TaskExecute {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async taskExecute({
    task,
    worker,
  }: TaskExecuteArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const taskData = await this.account.task.data(task);
    const feePDA = await this.account.fee.pda(taskData.daemon);
    return this.cronos.instruction.taskExecute({
      accounts: {
        clock: SYSVAR_CLOCK_PUBKEY,
        config: configPDA.address,
        daemon: taskData.daemon,
        fee: feePDA.address,
        task: task,
        worker: worker,
      },
      remainingAccounts: buildRemainingAccounts(taskData.ix, taskData.daemon),
    });
  }
}
