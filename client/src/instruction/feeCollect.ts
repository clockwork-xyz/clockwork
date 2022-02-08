import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type FeeCollectArgs = {
  fee: PublicKey;
  signer: PublicKey;
};

export class FeeCollect {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async feeCollect({
    fee,
    signer,
  }: FeeCollectArgs): Promise<TransactionInstruction> {
    const treasuryPDA = await this.account.treasury.pda();
    return this.cronos.instruction.feeCollect({
      accounts: {
        fee: fee,
        signer: signer,
        treasury: treasuryPDA.address,
      },
    });
  }
}
