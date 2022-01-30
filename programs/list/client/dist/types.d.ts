import { AccountData } from "@faktorfi/utils";
import { ListProgram } from "./idl";
export declare type ElementAccountData = AccountData<ListProgram["accounts"][0], ListProgram>;
export declare type ListAccountData = AccountData<ListProgram["accounts"][1], ListProgram>;
