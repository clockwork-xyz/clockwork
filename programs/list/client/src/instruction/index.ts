import { Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";

import { CreateList, CreateListArgs } from "./createList";
import { DeleteList, DeleteListArgs } from "./deleteList";
import { PushElement, PushElementArgs } from "./pushElement";
import { PopElement, PopElementArgs } from "./popElement";

export class Instruction {
  private account: Account;
  private program: Program<ListProgram>;

  public createList: (args: CreateListArgs) => Promise<TransactionInstruction>;

  public deleteList: (args: DeleteListArgs) => Promise<TransactionInstruction>;

  public pushElement: (
    args: PushElementArgs
  ) => Promise<TransactionInstruction>;

  public popElement: (args: PopElementArgs) => Promise<TransactionInstruction>;

  constructor(account: Account, program: Program<ListProgram>) {
    this.account = account;
    this.program = program;

    this.createList = new CreateList(account, program).createList;
    this.deleteList = new DeleteList(account, program).deleteList;
    this.popElement = new PopElement(account, program).popElement;
    this.pushElement = new PushElement(account, program).pushElement;
  }
}
