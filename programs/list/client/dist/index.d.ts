import { Provider } from "@project-serum/anchor";
import { Account } from "./account";
import { Instruction } from "./instruction";
export declare const PROGRAM_ID = "DBMi4GBjiX15vCMVj93uB7JYM9LU6rCaZJraVKM6XgZi";
export declare class ListProgram {
    static programId: string;
    account: Account;
    instruction: Instruction;
    constructor(provider: Provider);
}
export * from "./types";
