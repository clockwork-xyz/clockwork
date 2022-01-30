import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";

export type DeleteListArgs = {
  list: PublicKey;
};

export class DeleteList {
  private account: Account;
  private program: Program<ListProgram>;

  constructor(account: Account, program: Program<ListProgram>) {
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
