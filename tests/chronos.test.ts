import assert from "assert";
import {
  airdrop,
  dateToSeconds,
  findPDA,
  newSigner,
  PDA,
  signAndSubmit,
  sleep,
  sleepUntil,
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
import { ListProgram } from "@faktorfi/list-program";

type TaskRecurrenceSchedule = TypeDef<Chronos["types"][0], Chronos>;
type InstructionData = TypeDef<Chronos["types"][1], Chronos>;
type AccountMetaData = TypeDef<Chronos["types"][2], Chronos>;
type TaskStatus = TypeDef<Chronos["types"][3], Chronos>;

const SEED_AGENT = Buffer.from("daemon");
const SEED_AUTHORITY = Buffer.from("authority");
const SEED_FRAME = Buffer.from("frame");
const SEED_TASK = Buffer.from("task");

const ONE_MINUTE = new anchor.BN(60);

describe("Chronos", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  let listProgram = new ListProgram(provider);

  let signer: web3.Keypair;
  let worker: web3.Keypair;
  let authorityPDA: PDA;
  let daemonPDA: PDA;
  let frame0PDA: PDA;
  let frame1PDA: PDA;
  let list0PDA: PDA;
  let list1PDA: PDA;
  let taskA0PDA: PDA;
  let taskB0PDA: PDA;
  let taskB1PDA: PDA;
  let taskElementA0PDA: PDA;
  let taskElementB0PDA: PDA;
  let taskElementB1PDA: PDA;
  let signerTokens: PublicKey;
  let daemonTokens: PublicKey;
  let tokenProgram: Token;
  let timestamp: anchor.BN;
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

  it("Initializes", async () => {
    authorityPDA = await findPDA([SEED_AUTHORITY], program.programId);
    let ix = program.instruction.initialize(authorityPDA.bump, {
      accounts: {
        authority: authorityPDA.address,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
    await signAndSubmit(provider.connection, [ix], signer);

    const authorityData = await program.account.authority.fetch(
      authorityPDA.address
    );
    assert(authorityData.bump == authorityPDA.bump);
  });

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
      0
    );
    daemonTokens = await Token.createWrappedNativeAccount(
      provider.connection,
      TOKEN_PROGRAM_ID,
      daemonPDA.address,
      signer,
      LAMPORTS_PER_SOL * 0.3
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

  it("Creates a frame", async () => {
    timestamp = nextFrameTimestamp();
    frame0PDA = await findPDA(
      [SEED_FRAME, timestamp.toArrayLike(Buffer, "be", 8)],
      program.programId
    );
    list0PDA = await listProgram.account.list.pda(
      authorityPDA.address,
      frame0PDA.address
    );
    let ix = program.instruction.frameCreate(
      timestamp,
      frame0PDA.bump,
      list0PDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          clock: SYSVAR_CLOCK_PUBKEY,
          frame: frame0PDA.address,
          list: list0PDA.address,
          listProgram: ListProgram.programId,
          payer: signer.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    await signAndSubmit(provider.connection, [ix], signer);

    const frameData = await program.account.frame.fetch(frame0PDA.address);
    assert(frameData.timestamp.eq(timestamp));
    assert(frameData.bump == frame0PDA.bump);
  });

  it("Schedules a one-time task", async () => {
    const daemonData = await program.account.daemon.fetch(daemonPDA.address);
    taskA0PDA = await findPDA(
      [
        SEED_TASK,
        daemonPDA.address.toBuffer(),
        daemonData.totalTaskCount.toArrayLike(Buffer, "be", 16),
      ],
      program.programId
    );

    let listData = await listProgram.account.list.data(list0PDA.address);
    taskElementA0PDA = await listProgram.account.element.pda(
      list0PDA.address,
      listData.count
    );

    // Create SPL token transfer instruction
    let taskIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );

    // Schedule a one-time task
    let ix = program.instruction.taskSchedule(
      buildInstructionData(taskIx),
      timestamp,
      new anchor.BN(0),
      timestamp,
      taskA0PDA.bump,
      taskElementA0PDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          daemon: daemonPDA.address,
          frame: frame0PDA.address,
          listProgram: ListProgram.programId,
          task: taskA0PDA.address,
          taskElement: taskElementA0PDA.address,
          taskList: list0PDA.address,
          owner: signer.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );
    await signAndSubmit(provider.connection, [ix], signer);

    let taskData = await program.account.task.fetch(taskA0PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.executeAt.eq(timestamp));
    assert(taskData.repeatEvery.eq(new anchor.BN(0)));
    assert(taskData.repeatUntil.eq(timestamp));
    assert(taskData.bump === taskA0PDA.bump);
  });

  it("Schedules a recurring task", async () => {
    const daemonData = await program.account.daemon.fetch(daemonPDA.address);
    taskB0PDA = await findPDA(
      [
        SEED_TASK,
        daemonPDA.address.toBuffer(),
        daemonData.totalTaskCount.toArrayLike(Buffer, "be", 16),
      ],
      program.programId
    );

    let listData = await listProgram.account.list.data(list0PDA.address);
    taskElementB0PDA = await listProgram.account.element.pda(
      list0PDA.address,
      listData.count
    );

    // Create SPL token transfer instruction
    let taskIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );

    // Schedule a one-time task
    let ix = program.instruction.taskSchedule(
      buildInstructionData(taskIx),
      timestamp,
      ONE_MINUTE,
      timestamp.add(ONE_MINUTE),
      taskB0PDA.bump,
      taskElementB0PDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          daemon: daemonPDA.address,
          frame: frame0PDA.address,
          listProgram: ListProgram.programId,
          task: taskB0PDA.address,
          taskElement: taskElementB0PDA.address,
          taskList: list0PDA.address,
          owner: signer.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );
    await signAndSubmit(provider.connection, [ix], signer);

    let taskData = await program.account.task.fetch(taskB0PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.executeAt.eq(timestamp));
    assert(taskData.repeatEvery.eq(ONE_MINUTE));
    assert(taskData.repeatUntil.eq(timestamp.add(ONE_MINUTE)));
    assert(taskData.bump === taskB0PDA.bump);
  });

  it("Executes a one-time task", async () => {
    // Measure balances before
    await sleepUntil(new Date(timestamp.toNumber() * 1000 + 2000));
    let daemonTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      daemonTokens
    );
    let signerTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      signerTokens
    );

    // Process task
    let taskData = await program.account.task.fetch(taskA0PDA.address);
    let ix = program.instruction.taskExecute({
      accounts: {
        clock: SYSVAR_CLOCK_PUBKEY,
        daemon: taskData.daemon,
        task: taskA0PDA.address,
        worker: worker.publicKey,
      },
      remainingAccounts: buildRemainingAccounts(
        taskData.instructionData,
        taskData.daemon
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
    taskData = await program.account.task.fetch(taskA0PDA.address);
    assert(Object.keys(taskData.status)[0] === "done");
  });

  it("Executes a recurring task", async () => {
    // Measure balances before
    await sleepUntil(new Date(timestamp.toNumber() * 1000 + 2000));
    let daemonTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      daemonTokens
    );
    let signerTokenAccountInfoBefore = await tokenProgram.getAccountInfo(
      signerTokens
    );

    // Process task
    let taskData = await program.account.task.fetch(taskB0PDA.address);
    let ix = program.instruction.taskExecute({
      accounts: {
        clock: SYSVAR_CLOCK_PUBKEY,
        daemon: taskData.daemon,
        task: taskB0PDA.address,
        worker: worker.publicKey,
      },
      remainingAccounts: buildRemainingAccounts(
        taskData.instructionData,
        taskData.daemon
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
    taskData = await program.account.task.fetch(taskB0PDA.address);
    assert(Object.keys(taskData.status)[0] === "repeat");
  });

  // TODO repeat task
  it("Repeats a recurring task", async () => {
    // Process task
    let taskData = await program.account.task.fetch(taskB0PDA.address);
    let daemonData = await program.account.daemon.fetch(daemonPDA.address);

    let next_timestamp = taskData.executeAt.add(taskData.repeatEvery);
    frame1PDA = await findPDA(
      [SEED_FRAME, next_timestamp.toArrayLike(Buffer, "be", 8)],
      program.programId
    );
    list1PDA = await listProgram.account.list.pda(
      authorityPDA.address,
      frame1PDA.address
    );
    let ixA = program.instruction.frameCreate(
      next_timestamp,
      frame1PDA.bump,
      list1PDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          clock: SYSVAR_CLOCK_PUBKEY,
          frame: frame1PDA.address,
          list: list1PDA.address,
          listProgram: ListProgram.programId,
          payer: worker.publicKey,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    taskB1PDA = await findPDA(
      [
        SEED_TASK,
        daemonPDA.address.toBuffer(),
        daemonData.totalTaskCount.toArrayLike(Buffer, "be", 16),
      ],
      program.programId
    );
    taskElementB1PDA = await listProgram.account.element.pda(
      list1PDA.address,
      new anchor.BN(0)
    );

    let ixB = program.instruction.taskRepeat(
      taskB1PDA.bump,
      taskElementB1PDA.bump,
      {
        accounts: {
          authority: authorityPDA.address,
          daemon: taskData.daemon,
          listProgram: ListProgram.programId,
          nextFrame: frame1PDA.address,
          nextTask: taskB1PDA.address,
          nextTaskElement: taskElementB1PDA.address,
          nextTaskList: list1PDA.address,
          prevTask: taskB0PDA.address,
          systemProgram: SystemProgram.programId,
          worker: worker.publicKey,
        },
      }
    );
    await signAndSubmit(provider.connection, [ixA, ixB], worker);
  });
});

function nextFrameTimestamp(): anchor.BN {
  const now = new Date();
  const thisFrame = new Date(now.setSeconds(0, 0));
  const nextFrame = new Date(
    thisFrame.getTime() + ONE_MINUTE.toNumber() * 1000
  );
  return new anchor.BN(dateToSeconds(nextFrame));
}

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
