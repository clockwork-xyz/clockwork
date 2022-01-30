import { Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";
import { CreateListArgs } from "./createList";
import { DeleteListArgs } from "./deleteList";
import { PushElementArgs } from "./pushElement";
import { PopElementArgs } from "./popElement";
export declare class Instruction {
    private account;
    private program;
    createList: (args: CreateListArgs) => Promise<TransactionInstruction>;
    deleteList: (args: DeleteListArgs) => Promise<TransactionInstruction>;
    pushElement: (args: PushElementArgs) => Promise<TransactionInstruction>;
    popElement: (args: PopElementArgs) => Promise<TransactionInstruction>;
    constructor(account: Account, program: Program<ListProgram>);
}
