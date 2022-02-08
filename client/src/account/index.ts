import { Program } from "@project-serum/anchor";
import { Cronos } from "../idl";

import { AuthorityGateway } from "./authority";
import { ConfigGateway } from "./config";
import { DaemonGateway } from "./daemon";
import { HealthGateway } from "./health";
import { FeeGateway } from "./fee";
import { TaskGateway } from "./task";
import { TreasuryGateway } from "./treasury";

export class Account {
  public authority: AuthorityGateway;
  public config: ConfigGateway;
  public daemon: DaemonGateway;
  public health: HealthGateway;
  public fee: FeeGateway;
  public task: TaskGateway;
  public treasury: TreasuryGateway;

  constructor(cronos: Program<Cronos>) {
    this.authority = new AuthorityGateway(cronos, cronos.account.authority);
    this.config = new ConfigGateway(cronos, cronos.account.config);
    this.daemon = new DaemonGateway(cronos, cronos.account.daemon);
    this.health = new HealthGateway(cronos, cronos.account.health);
    this.fee = new FeeGateway(cronos, cronos.account.fee);
    this.task = new TaskGateway(cronos, cronos.account.task);
    this.treasury = new TreasuryGateway(cronos, cronos.account.treasury);
  }
}
