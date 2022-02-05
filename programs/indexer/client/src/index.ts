import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, Indexer as _Indexer } from "./idl";

export class Indexer {
  static DEVNET_PROGRAM_ID = "4jnuXU4zsBavciDpTNmJ8dtwMBSkhKn2EBCiE9Rrv8uQ";
  static MAINNET_PROGRAM_ID = "";

  public account: Account;
  public instruction: Instruction;
  public programId: string;

  constructor(provider: Provider, programId: string) {
    const program = new Program(IDL, programId, provider);
    const account = new Account(program);
    const instruction = new Instruction(account, program);
    this.account = account;
    this.instruction = instruction;
    this.programId = programId;
  }
}

export * from "./types";
