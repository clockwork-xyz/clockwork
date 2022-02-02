import { BN, Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";

export type PopElementArgs = {
  list: PublicKey;
};

export class PopElement {
  private account: Account;
  private program: Program<ListProgram>;

  constructor(account: Account, program: Program<ListProgram>) {
    this.account = account;
    this.program = program;
  }

  public async popElement({
    list,
  }: PopElementArgs): Promise<TransactionInstruction> {
    const listData = await this.account.list.data(list);
    const elementPDA = await this.account.element.pda(
      list,
      listData.count.sub(new BN(1))
    );
    return this.program.instruction.popElement({
      accounts: {
        list: list,
        element: elementPDA.address,
        owner: listData.owner,
      },
    });
  }
}
