import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { ListProgram } from "../idl";
import { Account } from "../account";
export declare type PushElementArgs = {
    list: PublicKey;
    owner: PublicKey;
    value: PublicKey;
};
export declare class PushElement {
    private account;
    private program;
    constructor(account: Account, program: Program<ListProgram>);
    pushElement({ list, owner, value, }: PushElementArgs): Promise<TransactionInstruction>;
}
