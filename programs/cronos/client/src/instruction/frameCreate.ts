import { BN, Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";
import { Indexer } from "@cronos-so/indexer";

export type FrameCreateArgs = {
  signer: PublicKey;
  timestamp: BN;
};

export class FrameCreate {
  private account: Account;
  private cronos: Program<Cronos>;
  private indexer: Indexer;

  constructor(account: Account, cronos: Program<Cronos>, indexer: Indexer) {
    this.account = account;
    this.cronos = cronos;
    this.indexer = indexer;
  }

  public async frameCreate({
    signer,
    timestamp,
  }: FrameCreateArgs): Promise<TransactionInstruction> {
    const authorityPDA = await this.account.authority.pda();
    const configPDA = await this.account.config.pda();
    const framePDA = await this.account.frame.pda(timestamp);
    const listPDA = await this.indexer.account.list.pda(
      authorityPDA.address,
      framePDA.address
    );
    return this.cronos.instruction.frameCreate(
      timestamp,
      framePDA.bump,
      listPDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          clock: SYSVAR_CLOCK_PUBKEY,
          config: configPDA.address,
          frame: framePDA.address,
          list: listPDA.address,
          indexerProgram: this.indexer.programId,
          payer: signer,
          systemProgram: SystemProgram.programId,
        },
      }
    );
  }
}
