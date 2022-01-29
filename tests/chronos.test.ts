import assert from "assert";
import {
  airdrop,
  findPDA,
  newSigner,
  PDA,
  signAndSubmit,
} from "@faktorfi/utils";
import * as anchor from "@project-serum/anchor";
import { web3, Program } from "@project-serum/anchor";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { Token, TOKEN_PROGRAM_ID, NATIVE_MINT } from "@solana/spl-token";
import { Chronos } from "../target/types/chronos";
import { SystemProgram, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

type InstructionData = TypeDef<Chronos["types"][0], Chronos>;
type AccountMetaData = TypeDef<Chronos["types"][1], Chronos>;

const SEED_AGENT = Buffer.from("agent");
const SEED_TASK = Buffer.from("task");

describe("Chronos", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  let signer: web3.Keypair;
  let worker: web3.Keypair;
  let agentPDA: PDA;
  let taskPDA: PDA;
  let signerTokens: PublicKey;
  let agentTokens: PublicKey;
  let tokenProgram: Token;
  let TRANSFER_AMOUNT = new anchor.BN(0.05 * LAMPORTS_PER_SOL);

  before(async () => {
    signer = await newSigner(provider.connection);
    worker = await newSigner(provider.connection);
    agentPDA = await findPDA(
      [SEED_AGENT, signer.publicKey.toBuffer()],
      program.programId
    );
    tokenProgram = new Token(
      provider.connection,
      NATIVE_MINT,
      TOKEN_PROGRAM_ID,
      signer
    );
  });

  const program = anchor.workspace.Chronos as Program<Chronos>;

  it("Creates an agent", async () => {
    let ix = program.instruction.agentCreate(agentPDA.bump, {
      accounts: {
        agent: agentPDA.address,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    await signAndSubmit(provider.connection, [ix], signer);

    let agentData = await program.account.agent.fetch(agentPDA.address);
    assert(agentData.owner.toString() === signer.publicKey.toString());
    assert(agentData.bump === agentPDA.bump);
  });

  before(async () => {
    signerTokens = await Token.createWrappedNativeAccount(
      provider.connection,
      TOKEN_PROGRAM_ID,
      signer.publicKey,
      signer,
      LAMPORTS_PER_SOL * 0.1
    );
    agentTokens = await Token.createWrappedNativeAccount(
      provider.connection,
      TOKEN_PROGRAM_ID,
      agentPDA.address,
      signer,
      LAMPORTS_PER_SOL * 0.1
    );
  });

  it("Creates an task", async () => {
    taskPDA = await findPDA(
      [SEED_TASK, agentPDA.address.toBuffer()],
      program.programId
    );

    // Create SPL token transfer instruction
    let taskIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      agentTokens,
      signerTokens,
      agentPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    let ix = program.instruction.taskCreate(taskIx, taskPDA.bump, {
      accounts: {
        agent: agentPDA.address,
        task: taskPDA.address,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    await signAndSubmit(provider.connection, [ix], signer);

    let taskData = await program.account.task.fetch(taskPDA.address);
    assert(taskData.agent.toString() === agentPDA.address.toString());
    assert(taskData.isProcessed === false);
    assert(taskData.bump === taskPDA.bump);
  });

  it("Processes a task", async () => {
    // Measure balances before
    let agentTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      agentTokens
    );
    let signerTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      signerTokens
    );

    // Process task
    let taskData = await program.account.task.fetch(taskPDA.address);
    let remainingAccounts = (
      taskData.instructionData.keys as AccountMetaData[]
    ).map((acc) => {
      if (acc.pubkey.toString() === agentPDA.address.toString())
        acc.isSigner = false;
      return acc;
    });
    remainingAccounts.push({
      pubkey: taskData.instructionData.programId,
      isSigner: false,
      isWritable: false,
    });
    let ix = program.instruction.taskProcess({
      accounts: {
        agent: agentPDA.address,
        task: taskPDA.address,
        worker: worker.publicKey,
      },
      remainingAccounts: remainingAccounts,
    });
    await signAndSubmit(provider.connection, [ix], worker);

    // Validate token balances after
    let agentTokenAccountInfoAfter = await tokenProgram.getAccountInfo(
      agentTokens
    );
    let signerTokenAccountInfoAfter = await tokenProgram.getAccountInfo(
      signerTokens
    );
    assert(
      agentTokenAccountInfoAfter.amount.eq(
        agentTokenAccountInfoBefore.amount.sub(TRANSFER_AMOUNT)
      )
    );
    assert(
      signerTokenAccountInfoAfter.amount.eq(
        signerTokenAccountInfoBefore.amount.add(TRANSFER_AMOUNT)
      )
    );

    // Validate task data
    taskData = await program.account.task.fetch(taskPDA.address);
    assert(taskData.isProcessed === true);
  });
});
