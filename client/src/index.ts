import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, Cronos as _Cronos } from "./idl";

export class Cronos {
  static DEVNET_PROGRAM_ID = "AUDt6tUdA3eMNjX5pyirrroNLyqupKUnzpeFdsrGBWmv";
  static MAINNET_PROGRAM_ID = "";

  public account: Account;
  public instruction: Instruction;
  public programId: string;

  constructor(provider: Provider, programId: string) {
    const cronos = new Program(IDL, programId, provider);
    const account = new Account(cronos);
    const instruction = new Instruction(account, cronos);
    this.account = account;
    this.instruction = instruction;
    this.programId = programId;
  }
}

export * from "./types";
