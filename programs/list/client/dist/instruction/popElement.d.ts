import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";
export declare type PopElementArgs = {
    list: PublicKey;
};
export declare class PopElement {
    private account;
    private program;
    constructor(account: Account, program: Program<ListProgram>);
    popElement({ list, }: PopElementArgs): Promise<TransactionInstruction>;
}
