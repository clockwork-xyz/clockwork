import { ListProgram } from "@chronos-so/indexer";
import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, Chronos as _Chronos } from "./idl";

export const PROGRAM_ID = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";

export class Chronos {
  static programId = PROGRAM_ID;

  public account: Account;
  public instruction: Instruction;

  constructor(provider: Provider) {
    const chronos = new Program(IDL, PROGRAM_ID, provider);
    const indexer = new ListProgram(provider);
    const account = new Account(chronos);
    const instruction = new Instruction(account, chronos, indexer);
    this.account = account;
    this.instruction = instruction;
  }
}

export * from "./types";
