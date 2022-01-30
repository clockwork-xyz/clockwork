import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, ListProgram as _ListProgram } from "./idl";

export const PROGRAM_ID = "DBMi4GBjiX15vCMVj93uB7JYM9LU6rCaZJraVKM6XgZi";

export class ListProgram {
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
