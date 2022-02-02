import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";
import { Indexer } from "@chronos-so/indexer";

export type TaskRepeatArgs = {
  task: PublicKey;
  worker: PublicKey;
};

export class TaskRepeat {
  private account: Account;
  private chronos: Program<Chronos>;
  private indexer: Indexer;

  constructor(account: Account, chronos: Program<Chronos>, indexer: Indexer) {
    this.account = account;
    this.chronos = chronos;
    this.indexer = indexer;
  }

  public async taskRepeat({
    task,
    worker,
  }: TaskRepeatArgs): Promise<TransactionInstruction> {
    const authorityPDA = await this.account.authority.pda();
    const configPDA = await this.account.config.pda();
    const taskData = await this.account.task.data(task);
    const nextTimestamp = taskData.executeAt.add(taskData.repeatEvery);
    const nextFramePDA = await this.account.frame.pda(nextTimestamp);
    const daemonData = await this.account.daemon.data(taskData.daemon);
    const nextTaskPDA = await this.account.task.pda(
      taskData.daemon,
      daemonData.totalTaskCount
    );
    const nextTaskListPDA = await this.indexer.account.list.pda(
      authorityPDA.address,
      nextFramePDA.address
    );
    const nextTaskListData = await this.indexer.account.list.data(
      nextTaskListPDA.address
    );
    const nextTaskElementPDA = await this.indexer.account.element.pda(
      nextTaskListPDA.address,
      nextTaskListData.count
    );
    return this.chronos.instruction.taskRepeat(
      nextTaskPDA.bump,
      nextTaskElementPDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          config: configPDA.address,
          daemon: taskData.daemon,
          indexerProgram: Indexer.programId,
          nextFrame: nextFramePDA.address,
          nextTask: nextTaskPDA.address,
          nextTaskElement: nextTaskElementPDA.address,
          nextTaskList: nextTaskListPDA.address,
          prevTask: task,
          systemProgram: SystemProgram.programId,
          worker: worker,
        },
      }
    );
  }
}
