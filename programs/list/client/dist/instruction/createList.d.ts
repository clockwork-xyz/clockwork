import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";
export declare type CreateListArgs = {
    namespace: PublicKey;
    owner: PublicKey;
    payer: PublicKey;
};
export declare class CreateList {
    private account;
    private program;
    constructor(account: Account, program: Program<ListProgram>);
    createList({ namespace, owner, payer, }: CreateListArgs): Promise<TransactionInstruction>;
}
