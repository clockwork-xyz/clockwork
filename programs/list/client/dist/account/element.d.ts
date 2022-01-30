import { Gateway } from "@faktorfi/utils";
import { BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { ListProgram } from "../idl";
export declare class ElementGateway extends Gateway<ListProgram, ListProgram["accounts"][0]> {
    pda(index: PublicKey, position: BN): Promise<import("@faktorfi/utils").PDA>;
}
