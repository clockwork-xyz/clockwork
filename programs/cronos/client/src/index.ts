import { Indexer } from "@cronos-so/indexer";
import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, Cronos as _Cronos } from "./idl";

export const PROGRAM_ID = "9cEqpQLV3VGN6mBtFKwheJoreg6BXvyCf6pWWDA1FhRf";

export class Cronos {
  static programId = PROGRAM_ID;

  public account: Account;
  public instruction: Instruction;

  constructor(provider: Provider) {
    const cronos = new Program(IDL, PROGRAM_ID, provider);
    const indexer = new Indexer(provider);
    const account = new Account(cronos);
    const instruction = new Instruction(account, cronos, indexer);
    this.account = account;
    this.instruction = instruction;
  }
}

export * from "./types";
