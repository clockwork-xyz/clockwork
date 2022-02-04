import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type RevenueCollectArgs = {
  revenue: PublicKey;
  signer: PublicKey;
};

export class RevenueCollect {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async revenueCollect({
    revenue,
    signer,
  }: RevenueCollectArgs): Promise<TransactionInstruction> {
    const treasuryPDA = await this.account.treasury.pda();
    return this.cronos.instruction.revenueCollect({
      accounts: {
        revenue: revenue,
        signer: signer,
        treasury: treasuryPDA.address,
      },
    });
  }
}
