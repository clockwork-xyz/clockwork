import { AccountData } from "@cronos-so/utils";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { Cronos } from "./idl";

// Account data
export type AuthorityAccountData = AccountData<Cronos["accounts"][0], Cronos>;
export type ConfigAccountData = AccountData<Cronos["accounts"][1], Cronos>;
export type DaemonAccountData = AccountData<Cronos["accounts"][2], Cronos>;
export type FrameAccountData = AccountData<Cronos["accounts"][3], Cronos>;
export type RevenueAccountData = AccountData<Cronos["accounts"][4], Cronos>;
export type TaskAccountData = AccountData<Cronos["accounts"][5], Cronos>;
export type TreasuryAccountData = AccountData<Cronos["accounts"][6], Cronos>;

// Types
export type InstructionData = TypeDef<Cronos["types"][0], Cronos>;
export type AccountMetaData = TypeDef<Cronos["types"][1], Cronos>;
export type TaskStatus = TypeDef<Cronos["types"][2], Cronos>;
