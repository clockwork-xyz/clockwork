import { BN, Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";

export type ConfigUpdateFrameIntervalArgs = {
  newFrameInterval: BN;
};

export class ConfigUpdateFrameInterval {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async configUpdateFrameInterval({
    newFrameInterval,
  }: ConfigUpdateFrameIntervalArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const configData = await this.account.config.data(configPDA.address);
    return this.chronos.instruction.configUpdateFrameInterval(
      newFrameInterval,
      {
        accounts: {
          admin: configData.adminAuthority,
          config: configPDA.address,
        },
      }
    );
  }
}
