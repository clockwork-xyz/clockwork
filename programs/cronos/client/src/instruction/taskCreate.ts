import { Indexer } from "@cronos-so/indexer";
import { BN, Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { buildInstructionData } from "./utils";
import { Cronos } from "../idl";
import { Account } from "../account";

export type TaskCreateArgs = {
  daemon: PublicKey;
  executeAt: BN;
  repeatEvery: BN;
  repeatUntil: BN;
  instruction: TransactionInstruction;
};

export class TaskCreate {
  private account: Account;
  private cronos: Program<Cronos>;
  private indexer: Indexer;

  constructor(account: Account, cronos: Program<Cronos>, indexer: Indexer) {
    this.account = account;
    this.cronos = cronos;
    this.indexer = indexer;
  }

  public async taskCreate({
    daemon,
    executeAt,
    repeatEvery,
    repeatUntil,
    instruction,
  }: TaskCreateArgs): Promise<TransactionInstruction> {
    const authorityPDA = await this.account.authority.pda();
    const framePDA = await this.account.frame.pda(executeAt);
    const taskListPDA = await this.indexer.account.list.pda(
      authorityPDA.address,
      framePDA.address
    );
    const daemonData = await this.account.daemon.data(daemon);
    const taskPDA = await this.account.task.pda(
      daemon,
      daemonData.totalTaskCount
    );
    const taskListData = await this.indexer.account.list.data(
      taskListPDA.address
    );
    const taskElementPDA = await this.indexer.account.element.pda(
      taskListPDA.address,
      taskListData.count
    );
    const instructionData = buildInstructionData(instruction);
    return this.cronos.instruction.taskCreate(
      instructionData,
      executeAt,
      repeatEvery,
      repeatUntil,
      taskPDA.bump,
      taskElementPDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          daemon: daemon,
          frame: framePDA.address,
          indexerProgram: Indexer.programId,
          task: taskPDA.address,
          taskElement: taskElementPDA.address,
          taskList: taskListPDA.address,
          owner: daemonData.owner,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  }
}
