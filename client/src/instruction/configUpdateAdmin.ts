import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type ConfigUpdateAdminArgs = {
  newAdmin: PublicKey;
};

export class ConfigUpdateAdmin {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async configUpdateAdmin({
    newAdmin,
  }: ConfigUpdateAdminArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const configData = await this.account.config.data(configPDA.address);
    return this.cronos.instruction.configUpdateAdmin(newAdmin, {
      accounts: {
        admin: configData.admin,
        config: configPDA.address,
      },
    });
  }
}
