import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { buildRemainingAccounts } from "./utils";
import { Chronos } from "../idl";
import { Account } from "../account";

export type TaskExecuteArgs = {
  task: PublicKey;
  worker: PublicKey;
};

export class TaskExecute {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async taskExecute({
    task,
    worker,
  }: TaskExecuteArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const taskData = await this.account.task.data(task);
    const revenuePDA = await this.account.revenue.pda(taskData.daemon);
    return this.chronos.instruction.taskExecute({
      accounts: {
        clock: SYSVAR_CLOCK_PUBKEY,
        config: configPDA.address,
        daemon: taskData.daemon,
        revenue: revenuePDA.address,
        task: task,
        worker: worker,
      },
      remainingAccounts: buildRemainingAccounts(
        taskData.instructionData,
        taskData.daemon
      ),
    });
  }
}
