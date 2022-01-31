import { Gateway } from "@chronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { ListProgram } from "../idl";
export declare class ListGateway extends Gateway<ListProgram, ListProgram["accounts"][1]> {
    pda(owner: PublicKey, namespace: PublicKey): Promise<import("@chronos-so/utils").PDA>;
}
