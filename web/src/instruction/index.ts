import { Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

import { AdminUpdateAdmin, AdminUpdateAdminArgs } from "./adminUpdateAdmin";
import {
  AdminUpdateProgramFee,
  AdminUpdateProgramFeeArgs,
} from "./adminUpdateProgramFee";
import {
  AdminUpdateWorkerFee,
  AdminUpdateWorkerFeeArgs,
} from "./adminUpdateWorkerFee";
import { DaemonCreate, DaemonCreateArgs } from "./daemonCreate";
import { DaemonInvoke, DaemonInvokeArgs } from "./daemonInvoke";
import { AdminInitialize, AdminInitializeArgs } from "./adminInitialize";
import { FeeCollect, FeeCollectArgs } from "./feeCollect";
import { TaskCreate, TaskCreateArgs } from "./taskCreate";
import { TaskExecute, TaskExecuteArgs } from "./taskExecute";

export class Instruction {
  private account: Account;
  private cronos: Program<Cronos>;

  public adminUpdateAdmin: (
    args: AdminUpdateAdminArgs
  ) => Promise<TransactionInstruction>;

  public adminUpdateProgramFee: (
    args: AdminUpdateProgramFeeArgs
  ) => Promise<TransactionInstruction>;

  public adminUpdateWorkerFee: (
    args: AdminUpdateWorkerFeeArgs
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

  public adminInitialize: (args: AdminInitializeArgs) => Promise<TransactionInstruction>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;

    this.adminUpdateAdmin = new AdminUpdateAdmin(
      this.account,
      this.cronos
    ).adminUpdateAdmin;

    this.adminUpdateProgramFee = new AdminUpdateProgramFee(
      this.account,
      this.cronos
    ).adminUpdateProgramFee;

    this.adminUpdateWorkerFee = new AdminUpdateWorkerFee(
      this.account,
      this.cronos
    ).adminUpdateWorkerFee;

    this.daemonCreate = new DaemonCreate(
      this.account,
      this.cronos
    ).daemonCreate;

    this.daemonInvoke = new DaemonInvoke(
      this.account,
      this.cronos
    ).daemonInvoke;

    this.adminInitialize = new AdminInitialize(this.account, this.cronos).adminInitialize;

    this.feeCollect = new FeeCollect(this.account, this.cronos).feeCollect;

    this.taskCreate = new TaskCreate(this.account, this.cronos).taskCreate;

    this.taskExecute = new TaskExecute(this.account, this.cronos).taskExecute;
  }
}
