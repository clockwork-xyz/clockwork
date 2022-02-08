import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type InitializeArgs = {
  signer: PublicKey;
};

export class Initialize {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async initialize({
    signer,
  }: InitializeArgs): Promise<TransactionInstruction> {
    const authorityPDA = await this.account.authority.pda();
    const configPDA = await this.account.config.pda();
    const treasuryPDA = await this.account.treasury.pda();
    return this.cronos.instruction.initialize(
      authorityPDA.bump,
      configPDA.bump,
      treasuryPDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          config: configPDA.address,
          signer: signer,
          systemProgram: SystemProgram.programId,
          treasury: treasuryPDA.address,
        },
      }
    );
  }
}
