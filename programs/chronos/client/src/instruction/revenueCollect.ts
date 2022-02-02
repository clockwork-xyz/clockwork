import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";

export type RevenueCollectArgs = {
  revenue: PublicKey;
  signer: PublicKey;
};

export class RevenueCollect {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async revenueCollect({
    revenue,
    signer,
  }: RevenueCollectArgs): Promise<TransactionInstruction> {
    const treasuryPDA = await this.account.treasury.pda();
    return this.chronos.instruction.revenueCollect({
      accounts: {
        revenue: revenue,
        signer: signer,
        treasury: treasuryPDA.address,
      },
    });
  }
}
