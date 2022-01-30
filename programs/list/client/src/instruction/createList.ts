import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";

export type CreateListArgs = {
  namespace: PublicKey;
  owner: PublicKey;
  payer: PublicKey;
};

export class CreateList {
  private account: Account;
  private program: Program<ListProgram>;

  constructor(account: Account, program: Program<ListProgram>) {
    this.account = account;
    this.program = program;
  }

  public async createList({
    namespace,
    owner,
    payer,
  }: CreateListArgs): Promise<TransactionInstruction> {
    const listPDA = await this.account.list.pda(owner, namespace);
    return this.program.instruction.createList(listPDA.bump, {
      accounts: {
        list: listPDA.address,
        namespace: namespace,
        owner: owner,
        payer: payer,
        systemProgram: SystemProgram.programId,
      },
    });
  }
}
