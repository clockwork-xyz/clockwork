import { Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

import { ConfigUpdateAdmin, ConfigUpdateAdminArgs } from "./configUpdateAdmin";
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
import { FeeCollect, FeeCollectArgs } from "./feeCollect";
import { TaskCreate, TaskCreateArgs } from "./taskCreate";
import { TaskExecute, TaskExecuteArgs } from "./taskExecute";

export class Instruction {
  private account: Account;
  private cronos: Program<Cronos>;

  public configUpdateAdmin: (
    args: ConfigUpdateAdminArgs
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

  public feeCollect: (args: FeeCollectArgs) => Promise<TransactionInstruction>;

  public taskCreate: (args: TaskCreateArgs) => Promise<TransactionInstruction>;

  public taskExecute: (
    args: TaskExecuteArgs
  ) => Promise<TransactionInstruction>;

  public initialize: (args: InitializeArgs) => Promise<TransactionInstruction>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;

    this.configUpdateAdmin = new ConfigUpdateAdmin(
      this.account,
      this.cronos
    ).configUpdateAdmin;

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

    this.feeCollect = new FeeCollect(this.account, this.cronos).feeCollect;

    this.taskCreate = new TaskCreate(this.account, this.cronos).taskCreate;

    this.taskExecute = new TaskExecute(this.account, this.cronos).taskExecute;
  }
}
