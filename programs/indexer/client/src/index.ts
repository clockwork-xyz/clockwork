import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, Indexer as _Indexer } from "./idl";

export const PROGRAM_ID = "4jnuXU4zsBavciDpTNmJ8dtwMBSkhKn2EBCiE9Rrv8uQ";

export class Indexer {
  static programId = PROGRAM_ID;

  public account: Account;
  public instruction: Instruction;

  constructor(provider: Provider) {
    const program = new Program(IDL, PROGRAM_ID, provider);
    const account = new Account(program);
    const instruction = new Instruction(account, program);
    this.account = account;
    this.instruction = instruction;
  }
}

export * from "./types";
