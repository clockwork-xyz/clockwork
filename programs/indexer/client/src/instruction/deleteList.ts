import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Indexer } from "../idl";
import { Account } from "../account";

export type DeleteListArgs = {
  list: PublicKey;
};

export class DeleteList {
  private account: Account;
  private program: Program<Indexer>;

  constructor(account: Account, program: Program<Indexer>) {
    this.account = account;
    this.program = program;
  }

  public async deleteList({
    list,
  }: DeleteListArgs): Promise<TransactionInstruction> {
    const listData = await this.account.list.data(list);
    return this.program.instruction.deleteList({
      accounts: {
        list: list,
        owner: listData.owner,
      },
    });
  }
}
