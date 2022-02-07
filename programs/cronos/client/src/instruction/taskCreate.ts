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

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async taskCreate({
    daemon,
    executeAt,
    repeatEvery,
    repeatUntil,
    instruction,
  }: TaskCreateArgs): Promise<TransactionInstruction> {
    const daemonData = await this.account.daemon.data(daemon);
    const taskPDA = await this.account.task.pda(daemon, daemonData.taskCount);
    const instructionData = buildInstructionData(instruction);
    return this.cronos.instruction.taskCreate(
      instructionData,
      executeAt,
      repeatEvery,
      repeatUntil,
      taskPDA.bump,
      {
        accounts: {
          daemon: daemon,
          task: taskPDA.address,
          owner: daemonData.owner,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  }
}
