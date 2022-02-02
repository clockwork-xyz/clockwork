import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Indexer } from "../idl";
import { Account } from "../account";

export type PushElementArgs = {
  list: PublicKey;
  owner: PublicKey;
  value: PublicKey;
};

export class PushElement {
  private account: Account;
  private program: Program<Indexer>;

  constructor(account: Account, program: Program<Indexer>) {
    this.account = account;
    this.program = program;
  }

  public async pushElement({
    list,
    owner,
    value,
  }: PushElementArgs): Promise<TransactionInstruction> {
    const listData = await this.account.list.data(list);
    const elementPDA = await this.account.element.pda(list, listData.count);
    return this.program.instruction.pushElement(value, elementPDA.bump, {
      accounts: {
        list: list,
        element: elementPDA.address,
        owner: owner,
        payer: owner,
        systemProgram: SystemProgram.programId,
      },
    });
  }
}
