import { AccountClient, Idl, IdlTypes, Program } from "@project-serum/anchor";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { IdlTypeDef } from "@project-serum/anchor/dist/cjs/idl";
import { PublicKey } from "@solana/web3.js";

export type AccountData<A extends IdlTypeDef, T extends Idl> = TypeDef<
  A,
  IdlTypes<T>
>;

type AccountDataCache<A extends IdlTypeDef, T extends Idl> = Record<
  string,
  AccountData<A, T>
>;

export type Listener<A extends IdlTypeDef, T extends Idl> = (
  data: AccountData<A, T>
) => void;

export type Unsubscribe = () => void;

export class Gateway<I extends Idl, A extends IdlTypeDef> {
  private cache: AccountDataCache<A, I>;
  private client: AccountClient<I, A>;
  program: Program<I>;

  constructor(
    program: Program<I>,
    client: AccountClient<I, A, TypeDef<A, IdlTypes<I>>>
  ) {
    this.cache = {};
    this.client = client;
    this.program = program;
  }

  // public async data(address: PublicKey): Promise<AccountData<A, I>> {
  //   // Get cache key.
  //   const key = address.toBase58();

  //   // Fetch account data from cache/network.
  //   let data: AccountData<A, I> | null = null;
  //   if (this.cache && this.cache[key]) return this.cache[key];
  //   else data = await this.client.fetch(address);
  //   if (!data) throw new Error("Account not found: " + address.toString());

  //   // Save account data to cache.
  //   this.cache[key] = data;

  //   // Update cache if on-chain data changes.
  //   const subscription = this.client.subscribe(address, "confirmed");
  //   subscription.on(
  //     "change",
  //     (newData: AccountData<A, I>) => (this.cache[key] = newData)
  //   );
  //   return data;
  // }

  public async data(address: PublicKey): Promise<AccountData<A, I>> {
    return await this.client.fetch(address);
  }

  public subscribe(address: PublicKey, fn: Listener<A, I>) {
    const subscription = this.client.subscribe(address, "confirmed");
    subscription.on("change", (data: AccountData<A, I>) => fn(data));
    return () => this.client.unsubscribe(address);
  }
}
