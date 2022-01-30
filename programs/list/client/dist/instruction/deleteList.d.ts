import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";
export declare type DeleteListArgs = {
    list: PublicKey;
};
export declare class DeleteList {
    private account;
    private program;
    constructor(account: Account, program: Program<ListProgram>);
    deleteList({ list, }: DeleteListArgs): Promise<TransactionInstruction>;
}
