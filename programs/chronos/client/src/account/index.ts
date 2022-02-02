import { Program } from "@project-serum/anchor";
import { Chronos } from "../idl";

import { AuthorityGateway } from "./authority";
import { ConfigGateway } from "./config";
import { DaemonGateway } from "./daemon";
import { FrameGateway } from "./frame";
import { RevenueGateway } from "./revenue";
import { TaskGateway } from "./task";
import { TreasuryGateway } from "./treasury";

export class Account {
  public authority: AuthorityGateway;
  public config: ConfigGateway;
  public daemon: DaemonGateway;
  public frame: FrameGateway;
  public revenue: RevenueGateway;
  public task: TaskGateway;
  public treasury: TreasuryGateway;

  constructor(chronos: Program<Chronos>) {
    this.authority = new AuthorityGateway(chronos, chronos.account.authority);
    this.config = new ConfigGateway(chronos, chronos.account.config);
    this.daemon = new DaemonGateway(chronos, chronos.account.daemon);
    this.frame = new FrameGateway(chronos, chronos.account.frame);
    this.revenue = new RevenueGateway(chronos, chronos.account.revenue);
    this.task = new TaskGateway(chronos, chronos.account.task);
    this.treasury = new TreasuryGateway(chronos, chronos.account.treasury);
  }
}
