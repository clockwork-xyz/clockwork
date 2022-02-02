import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";

export type ConfigUpdateAdminAuthorityArgs = {
  newAdminAuthority: PublicKey;
};

export class ConfigUpdateAdminAuthority {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async configUpdateAdminAuthority({
    newAdminAuthority,
  }: ConfigUpdateAdminAuthorityArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const configData = await this.account.config.data(configPDA.address);
    return this.chronos.instruction.configUpdateAdminAuthority(
      newAdminAuthority,
      {
        accounts: {
          admin: configData.adminAuthority,
          config: configPDA.address,
        },
      }
    );
  }
}
