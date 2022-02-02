import { Indexer } from "@chronos-so/indexer";
import { Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";

import {
  ConfigUpdateAdminAuthority,
  ConfigUpdateAdminAuthorityArgs,
} from "./configUpdateAdminAuthority";
import {
  ConfigUpdateFrameInterval,
  ConfigUpdateFrameIntervalArgs,
} from "./configUpdateFrameInterval";
import {
  ConfigUpdateProgramFee,
  ConfigUpdateProgramFeeArgs,
} from "./configUpdateProgramFee";
import {
  ConfigUpdateWorkerFee,
  ConfigUpdateWorkerFeeArgs,
} from "./configUpdateWorkerFee";
import { DaemonCreate, DaemonCreateArgs } from "./daemonCreate";
import { DaemonInvoke, DaemonInvokeArgs } from "./daemonInvoke";
import { Initialize, InitializeArgs } from "./initialize";
import { RevenueCollect, RevenueCollectArgs } from "./revenueCollect";
import { RevenueCreate, RevenueCreateArgs } from "./revenueCreate";
import { TaskCreate, TaskCreateArgs } from "./taskCreate";
import { TaskExecute, TaskExecuteArgs } from "./taskExecute";
import { TaskRepeat, TaskRepeatArgs } from "./taskRepeat";

export class Instruction {
  private account: Account;
  private chronos: Program<Chronos>;
  private indexer: Indexer;

  public configUpdateAdminAuthority: (
    args: ConfigUpdateAdminAuthorityArgs
  ) => Promise<TransactionInstruction>;

  public configUpdateFrameInterval: (
    args: ConfigUpdateFrameIntervalArgs
  ) => Promise<TransactionInstruction>;

  public configUpdateProgramFee: (
    args: ConfigUpdateProgramFeeArgs
  ) => Promise<TransactionInstruction>;

  public configUpdateWorkerFee: (
    args: ConfigUpdateWorkerFeeArgs
  ) => Promise<TransactionInstruction>;

  public daemonCreate: (
    args: DaemonCreateArgs
  ) => Promise<TransactionInstruction>;

  public daemonInvoke: (
    args: DaemonInvokeArgs
  ) => Promise<TransactionInstruction>;

  public revenueCollect: (
    args: RevenueCollectArgs
  ) => Promise<TransactionInstruction>;

  public revenueCreate: (
    args: RevenueCreateArgs
  ) => Promise<TransactionInstruction>;

  public taskCreate: (args: TaskCreateArgs) => Promise<TransactionInstruction>;

  public taskExecute: (
    args: TaskExecuteArgs
  ) => Promise<TransactionInstruction>;

  public taskRepeat: (args: TaskRepeatArgs) => Promise<TransactionInstruction>;

  public initialize: (args: InitializeArgs) => Promise<TransactionInstruction>;

  constructor(account: Account, chronos: Program<Chronos>, indexer: Indexer) {
    this.account = account;
    this.chronos = chronos;
    this.indexer = indexer;

    this.configUpdateAdminAuthority = new ConfigUpdateAdminAuthority(
      this.account,
      this.chronos
    ).configUpdateAdminAuthority;

    this.configUpdateFrameInterval = new ConfigUpdateFrameInterval(
      this.account,
      this.chronos
    ).configUpdateFrameInterval;

    this.configUpdateProgramFee = new ConfigUpdateProgramFee(
      this.account,
      this.chronos
    ).configUpdateProgramFee;

    this.configUpdateWorkerFee = new ConfigUpdateWorkerFee(
      this.account,
      this.chronos
    ).configUpdateWorkerFee;

    this.daemonCreate = new DaemonCreate(
      this.account,
      this.chronos
    ).daemonCreate;

    this.daemonInvoke = new DaemonInvoke(
      this.account,
      this.chronos
    ).daemonInvoke;

    this.initialize = new Initialize(this.account, this.chronos).initialize;

    this.revenueCollect = new RevenueCollect(
      this.account,
      this.chronos
    ).revenueCollect;

    this.revenueCreate = new RevenueCreate(
      this.account,
      this.chronos
    ).revenueCreate;

    this.taskCreate = new TaskCreate(
      this.account,
      this.chronos,
      this.indexer
    ).taskCreate;

    this.taskExecute = new TaskExecute(this.account, this.chronos).taskExecute;

    this.taskRepeat = new TaskRepeat(
      this.account,
      this.chronos,
      this.indexer
    ).taskRepeat;
  }
}
