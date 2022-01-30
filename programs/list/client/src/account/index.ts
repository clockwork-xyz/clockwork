import { Program } from "@project-serum/anchor";
import { ListProgram } from "../idl";

import { ListGateway } from "./list";
import { ElementGateway } from "./element";

export class Account {
  public element: ElementGateway;
  public list: ListGateway;

  constructor(program: Program<ListProgram>) {
    this.element = new ElementGateway(program, program.account.element);
    this.list = new ListGateway(program, program.account.list);
  }
}
