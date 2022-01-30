import { Program } from "@project-serum/anchor";
import { ListProgram } from "../idl";
import { ListGateway } from "./list";
import { ElementGateway } from "./element";
export declare class Account {
    element: ElementGateway;
    list: ListGateway;
    constructor(program: Program<ListProgram>);
}
