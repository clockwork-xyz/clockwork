import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type TaskRepeatArgs = {
  task: PublicKey;
  worker: PublicKey;
};

export class TaskRepeat {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async taskRepeat({
    task,
    worker,
  }: TaskRepeatArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const taskData = await this.account.task.data(task);
    const daemonData = await this.account.daemon.data(taskData.daemon);
    const nextTaskPDA = await this.account.task.pda(
      taskData.daemon,
      daemonData.taskCount
    );
    return this.cronos.instruction.taskRepeat(nextTaskPDA.bump, {
      accounts: {
        config: configPDA.address,
        daemon: taskData.daemon,
        nextTask: nextTaskPDA.address,
        prevTask: task,
        systemProgram: SystemProgram.programId,
        worker: worker,
      },
    });
  }
}
