import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type AdminInitializeArgs = {
  signer: PublicKey;
};

export class AdminInitialize {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async adminInitialize({
    signer,
  }: AdminInitializeArgs): Promise<TransactionInstruction> {
    const authorityPDA = await this.account.authority.pda();
    const configPDA = await this.account.config.pda();
    const daemonPDA = await this.account.daemon.pda(authorityPDA.address);
    const feePDA = await this.account.fee.pda(daemonPDA.address);
    const healthPDA = await this.account.health.pda();
    const treasuryPDA = await this.account.treasury.pda();
    return this.cronos.instruction.adminInitialize(
      authorityPDA.bump,
      configPDA.bump,
      daemonPDA.bump,
      feePDA.bump,
      healthPDA.bump,
      treasuryPDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          config: configPDA.address,
          daemon: daemonPDA.address,
          fee: feePDA.address,
          health: healthPDA.address,
          signer: signer,
          systemProgram: SystemProgram.programId,
          treasury: treasuryPDA.address,
        },
      }
    );
  }
}
