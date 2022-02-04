import { BN, Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type ConfigUpdateFrameIntervalArgs = {
  newFrameInterval: BN;
};

export class ConfigUpdateFrameInterval {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async configUpdateFrameInterval({
    newFrameInterval,
  }: ConfigUpdateFrameIntervalArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const configData = await this.account.config.data(configPDA.address);
    return this.cronos.instruction.configUpdateFrameInterval(newFrameInterval, {
      accounts: {
        admin: configData.adminAuthority,
        config: configPDA.address,
      },
    });
  }
}
