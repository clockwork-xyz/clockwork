import assert from "assert";
import {
  airdrop,
  dateToSeconds,
  findPDA,
  newSigner,
  PDA,
  signAndSubmit,
  sleep,
} from "@faktorfi/utils";
import * as anchor from "@project-serum/anchor";
import { web3, Program } from "@project-serum/anchor";
import { TypeDef } from "@project-serum/anchor/dist/cjs/program/namespace/types";
import { Token, TOKEN_PROGRAM_ID, NATIVE_MINT } from "@solana/spl-token";
import { Chronos } from "../target/types/chronos";
import {
  SystemProgram,
  LAMPORTS_PER_SOL,
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
} from "@solana/web3.js";

type InstructionData = TypeDef<Chronos["types"][0], Chronos>;
type AccountMetaData = TypeDef<Chronos["types"][1], Chronos>;

const SEED_AGENT = Buffer.from("daemon");
const SEED_TASK = Buffer.from("task");

describe("Chronos", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  let signer: web3.Keypair;
  let worker: web3.Keypair;
  let daemonPDA: PDA;
  let taskPDA: PDA;
  let signerTokens: PublicKey;
  let daemonTokens: PublicKey;
  let tokenProgram: Token;
  let TRANSFER_AMOUNT = new anchor.BN(0.05 * LAMPORTS_PER_SOL);

  before(async () => {
    signer = await newSigner(provider.connection);
    worker = await newSigner(provider.connection);
    daemonPDA = await findPDA(
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

  it("Creates a daemon", async () => {
    let ix = program.instruction.daemonCreate(daemonPDA.bump, {
      accounts: {
        daemon: daemonPDA.address,
        owner: signer.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    await signAndSubmit(provider.connection, [ix], signer);

    let daemonData = await program.account.daemon.fetch(daemonPDA.address);
    assert(daemonData.owner.toString() === signer.publicKey.toString());
    assert(daemonData.bump === daemonPDA.bump);
  });

  before(async () => {
    signerTokens = await Token.createWrappedNativeAccount(
      provider.connection,
      TOKEN_PROGRAM_ID,
      signer.publicKey,
      signer,
      LAMPORTS_PER_SOL * 0.1
    );
    daemonTokens = await Token.createWrappedNativeAccount(
      provider.connection,
      TOKEN_PROGRAM_ID,
      daemonPDA.address,
      signer,
      LAMPORTS_PER_SOL * 0.1
    );
  });

  it("Invokes a daemon", async () => {
    // Measure balances before
    let daemonTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      daemonTokens
    );
    let signerTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      signerTokens
    );

    // Invoke a task
    let taskIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    const taskIxData = buildInstructionData(taskIx);
    let ix = program.instruction.daemonInvoke(taskIxData, {
      accounts: {
        daemon: daemonPDA.address,
        owner: signer.publicKey,
      },
      remainingAccounts: buildRemainingAccounts(taskIxData, daemonPDA.address),
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate token balances after
    let daemonTokenAccountInfoAfter = await tokenProgram.getAccountInfo(
      daemonTokens
    );
    let signerTokenAccountInfoAfter = await tokenProgram.getAccountInfo(
      signerTokens
    );
    assert(
      daemonTokenAccountInfoAfter.amount.eq(
        daemonTokenAccountInfoBefore.amount.sub(TRANSFER_AMOUNT)
      )
    );
    assert(
      signerTokenAccountInfoAfter.amount.eq(
        signerTokenAccountInfoBefore.amount.add(TRANSFER_AMOUNT)
      )
    );
  });

  it("Schedules a task", async () => {
    taskPDA = await findPDA(
      [SEED_TASK, daemonPDA.address.toBuffer()],
      program.programId
    );

    let now = new Date();
    let executeAt = new anchor.BN(dateToSeconds(now));

    // Create SPL token transfer instruction
    let taskIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    let ix = program.instruction.taskSchedule(
      buildInstructionData(taskIx),
      new anchor.BN(executeAt),
      taskPDA.bump,
      {
        accounts: {
          daemon: daemonPDA.address,
          task: taskPDA.address,
          owner: signer.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );
    await signAndSubmit(provider.connection, [ix], signer);

    let taskData = await program.account.task.fetch(taskPDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(taskData.isExecuted === false);
    assert(taskData.executeAt.eq(executeAt));
    assert(taskData.bump === taskPDA.bump);
  });

  it("Executes a task", async () => {
    // Measure balances before
    await sleep(500);
    let daemonTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      daemonTokens
    );
    let signerTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      signerTokens
    );

    // Process task
    let taskData = await program.account.task.fetch(taskPDA.address);
    let ix = program.instruction.taskExecute({
      accounts: {
        clock: SYSVAR_CLOCK_PUBKEY,
        daemon: daemonPDA.address,
        task: taskPDA.address,
        worker: worker.publicKey,
      },
      remainingAccounts: buildRemainingAccounts(
        taskData.instructionData,
        daemonPDA.address
      ),
    });
    await signAndSubmit(provider.connection, [ix], worker);

    // Validate token balances after
    let daemonTokenAccountInfoAfter = await tokenProgram.getAccountInfo(
      daemonTokens
    );
    let signerTokenAccountInfoAfter = await tokenProgram.getAccountInfo(
      signerTokens
    );
    assert(
      daemonTokenAccountInfoAfter.amount.eq(
        daemonTokenAccountInfoBefore.amount.sub(TRANSFER_AMOUNT)
      )
    );
    assert(
      signerTokenAccountInfoAfter.amount.eq(
        signerTokenAccountInfoBefore.amount.add(TRANSFER_AMOUNT)
      )
    );

    // Validate task data
    taskData = await program.account.task.fetch(taskPDA.address);
    assert(taskData.isExecuted === true);
  });
});

function buildInstructionData(
  ix: web3.TransactionInstruction
): InstructionData {
  return {
    programId: ix.programId,
    keys: ix.keys as Array<AccountMetaData>,
    data: ix.data,
  };
}

function buildRemainingAccounts(
  ixData: InstructionData,
  daemon: PublicKey
): Array<AccountMetaData> {
  return (ixData.keys as Array<AccountMetaData>)
    .map((acc) => ({
      pubkey: acc.pubkey,
      isSigner:
        acc.pubkey.toString() === daemon.toString() ? false : acc.isSigner,
      isWritable: acc.isWritable,
    }))
    .concat([
      {
        pubkey: ixData.programId,
        isSigner: false,
        isWritable: false,
      },
    ]);
}
