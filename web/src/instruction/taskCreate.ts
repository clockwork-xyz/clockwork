import { BN, Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { buildInstructionData } from "./utils";
import { Cronos } from "../idl";
import { Account } from "../account";

export type TaskCreateArgs = {
  daemon: PublicKey;
  ix: TransactionInstruction;
  execAt: BN;
  stopAt: BN;
  recurr: BN;
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
    ix,
    execAt,
    stopAt,
    recurr,
  }: TaskCreateArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const daemonData = await this.account.daemon.data(daemon);
    const taskPDA = await this.account.task.pda(daemon, daemonData.taskCount);
    const instructionData = buildInstructionData(ix);
    return this.cronos.instruction.taskCreate(
      instructionData,
      execAt,
      stopAt,
      recurr,
      taskPDA.bump,
      {
        accounts: {
          clock: SYSVAR_CLOCK_PUBKEY,
          config: configPDA.address,
          daemon: daemon,
          task: taskPDA.address,
          owner: daemonData.owner,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  }
}
