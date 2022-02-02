import { AccountData } from "@chronos-so/utils";
import { Indexer } from "./idl";

export type ElementAccountData = AccountData<Indexer["accounts"][0], Indexer>;

export type ListAccountData = AccountData<Indexer["accounts"][1], Indexer>;
