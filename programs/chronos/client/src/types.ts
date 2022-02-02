import { AccountData } from "@chronos-so/utils";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { Chronos } from "./idl";

// Account data
export type AuthorityAccountData = AccountData<Chronos["accounts"][0], Chronos>;
export type ConfigAccountData = AccountData<Chronos["accounts"][1], Chronos>;
export type DaemonAccountData = AccountData<Chronos["accounts"][2], Chronos>;
export type FrameAccountData = AccountData<Chronos["accounts"][3], Chronos>;
export type RevenueAccountData = AccountData<Chronos["accounts"][4], Chronos>;
export type TaskAccountData = AccountData<Chronos["accounts"][5], Chronos>;
export type TreasuryAccountData = AccountData<Chronos["accounts"][6], Chronos>;

// Types
export type InstructionData = TypeDef<Chronos["types"][0], Chronos>;
export type AccountMetaData = TypeDef<Chronos["types"][1], Chronos>;
export type TaskStatus = TypeDef<Chronos["types"][2], Chronos>;
