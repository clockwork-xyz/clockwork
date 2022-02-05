import { Indexer } from "@cronos-so/indexer";
import { Program, Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
import { IDL, Cronos as _Cronos } from "./idl";

export class Cronos {
  static DEVNET_PROGRAM_ID = "9cEqpQLV3VGN6mBtFKwheJoreg6BXvyCf6pWWDA1FhRf";
  static MAINNET_PROGRAM_ID = "";

  public account: Account;
  public instruction: Instruction;
  public programId: string;

  constructor(provider: Provider, programId: string) {
    const cronos = new Program(IDL, programId, provider);
    const indexer = new Indexer(
      provider,
      programId === Cronos.MAINNET_PROGRAM_ID
        ? Indexer.MAINNET_PROGRAM_ID
        : Indexer.DEVNET_PROGRAM_ID
    );
    const account = new Account(cronos);
    const instruction = new Instruction(account, cronos, indexer);
    this.account = account;
    this.instruction = instruction;
    this.programId = programId;
  }
}

export * from "./types";
