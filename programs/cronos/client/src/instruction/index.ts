import { Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

import {
  ConfigUpdateAdminAuthority,
  ConfigUpdateAdminAuthorityArgs,
} from "./configUpdateAdminAuthority";
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
import { FrameCreate, FrameCreateArgs } from "./frameCreate";
import { RevenueCollect, RevenueCollectArgs } from "./revenueCollect";
import { RevenueCreate, RevenueCreateArgs } from "./revenueCreate";
import { TaskCreate, TaskCreateArgs } from "./taskCreate";
import { TaskExecute, TaskExecuteArgs } from "./taskExecute";
import { TaskRepeat, TaskRepeatArgs } from "./taskRepeat";

export class Instruction {
  private account: Account;
  private cronos: Program<Cronos>;

  public configUpdateAdminAuthority: (
    args: ConfigUpdateAdminAuthorityArgs
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

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;

    this.configUpdateAdminAuthority = new ConfigUpdateAdminAuthority(
      this.account,
      this.cronos
    ).configUpdateAdminAuthority;

    this.configUpdateProgramFee = new ConfigUpdateProgramFee(
      this.account,
      this.cronos
    ).configUpdateProgramFee;

    this.configUpdateWorkerFee = new ConfigUpdateWorkerFee(
      this.account,
      this.cronos
    ).configUpdateWorkerFee;

    this.daemonCreate = new DaemonCreate(
      this.account,
      this.cronos
    ).daemonCreate;

    this.daemonInvoke = new DaemonInvoke(
      this.account,
      this.cronos
    ).daemonInvoke;

    this.initialize = new Initialize(this.account, this.cronos).initialize;

    this.revenueCollect = new RevenueCollect(
      this.account,
      this.cronos
    ).revenueCollect;

    this.revenueCreate = new RevenueCreate(
      this.account,
      this.cronos
    ).revenueCreate;

    this.taskCreate = new TaskCreate(this.account, this.cronos).taskCreate;

    this.taskExecute = new TaskExecute(this.account, this.cronos).taskExecute;

    this.taskRepeat = new TaskRepeat(this.account, this.cronos).taskRepeat;
  }
}
