import assert from "assert";
import {
  dateToSeconds,
  newSigner,
  PDA,
  signAndSubmit,
  sleepUntil,
} from "@cronos-so/utils";
import { BN, Provider, setProvider } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID, NATIVE_MINT } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Cronos } from "./";

const ONE_MINUTE = new BN(60);

describe("Cronos", () => {
  // Configure the client to use the local cluster.
  const provider = Provider.env();
  setProvider(provider);

  let cronos = new Cronos(provider, Cronos.DEVNET_PROGRAM_ID);

  let signer: Keypair;
  let worker: Keypair;
  let authorityPDA: PDA;
  let daemonPDA: PDA;
  let revenuePDA: PDA;
  let frame0PDA: PDA;
  let task1PDA: PDA;
  let task2PDA: PDA;
  let signerTokens: PublicKey;
  let daemonTokens: PublicKey;
  let tokenProgram: Token;
  let timestamp: BN;
  let TRANSFER_AMOUNT = new BN(0.05 * LAMPORTS_PER_SOL);

  before(async () => {
    signer = await newSigner(provider.connection);
    worker = await newSigner(provider.connection);
    daemonPDA = await cronos.account.daemon.pda(signer.publicKey);
    tokenProgram = new Token(
      provider.connection,
      NATIVE_MINT,
      TOKEN_PROGRAM_ID,
      signer
    );
  });

  it("Initializes", async () => {
    // Submit instruction.
    const ix = await cronos.instruction.initialize({
      signer: signer.publicKey,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate authority account.
    authorityPDA = await cronos.account.authority.pda();
    const authorityData = await cronos.account.authority.data(
      authorityPDA.address
    );
    assert(authorityData.bump == authorityPDA.bump);

    // TODO Validate config account.
    // TODO Validate treasury account.
  });

  it("Creates a daemon", async () => {
    // Submit instruction.
    let ix = await cronos.instruction.daemonCreate({
      owner: signer.publicKey,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate daemon account.
    let daemonData = await cronos.account.daemon.data(daemonPDA.address);
    assert(daemonData.owner.toString() === signer.publicKey.toString());
    assert(daemonData.bump === daemonPDA.bump);
  });

  it("Creates a revenue account", async () => {
    // Submit instruction.
    let ix = await cronos.instruction.revenueCreate({
      daemon: daemonPDA.address,
      signer: signer.publicKey,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate revenue account.
    revenuePDA = await cronos.account.revenue.pda(daemonPDA.address);
    let revenueData = await cronos.account.revenue.data(revenuePDA.address);
    assert(revenueData.balance.eq(new BN(0)));
    assert(revenueData.daemon.toString() === daemonPDA.address.toString());
    assert(revenueData.bump === revenuePDA.bump);
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
    let transferIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    const ix = await cronos.instruction.daemonInvoke({
      daemon: daemonPDA.address,
      instruction: transferIx,
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
    // Submit instruction.
    timestamp = nextFrameTimestamp();
    const ix = await cronos.instruction.frameCreate({
      signer: signer.publicKey,
      timestamp: timestamp,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate frame account.
    frame0PDA = await cronos.account.frame.pda(timestamp);
    const frameData = await cronos.account.frame.data(frame0PDA.address);
    assert(frameData.timestamp.eq(timestamp));
    assert(frameData.bump == frame0PDA.bump);
  });

  it("Schedules a one-time task", async () => {
    // Submit instruction
    const transferIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    const ix = await cronos.instruction.taskCreate({
      daemon: daemonPDA.address,
      executeAt: timestamp,
      repeatEvery: new BN(0),
      repeatUntil: timestamp,
      instruction: transferIx,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate task data.
    task1PDA = await cronos.account.task.pda(daemonPDA.address, new BN(0));
    let taskData = await cronos.account.task.data(task1PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.executeAt.eq(timestamp));
    assert(taskData.repeatEvery.eq(new BN(0)));
    assert(taskData.repeatUntil.eq(timestamp));
    assert(taskData.bump === task1PDA.bump);
  });

  it("Schedules a recurring task", async () => {
    // Submit instruction.
    const transferIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    const ix = await cronos.instruction.taskCreate({
      daemon: daemonPDA.address,
      executeAt: timestamp,
      repeatEvery: ONE_MINUTE,
      repeatUntil: timestamp.add(ONE_MINUTE),
      instruction: transferIx,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate task data.
    task2PDA = await cronos.account.task.pda(daemonPDA.address, new BN(1));
    let taskData = await cronos.account.task.data(task2PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.executeAt.eq(timestamp));
    assert(taskData.repeatEvery.eq(ONE_MINUTE));
    assert(taskData.repeatUntil.eq(timestamp.add(ONE_MINUTE)));
    assert(taskData.bump === task2PDA.bump);
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

    // Sumbit instruction.
    const ix = await cronos.instruction.taskExecute({
      task: task1PDA.address,
      worker: worker.publicKey,
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
    const taskData = await cronos.account.task.data(task1PDA.address);
    assert(Object.keys(taskData.status)[0] === "executed");
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
    const ix = await cronos.instruction.taskExecute({
      task: task2PDA.address,
      worker: worker.publicKey,
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
    const taskData = await cronos.account.task.data(task2PDA.address);
    assert(Object.keys(taskData.status)[0] === "repeatable");
  });

  it("Repeats a recurring task", async () => {
    // Submit instructions
    const taskData = await cronos.account.task.data(task2PDA.address);
    const nextTimestamp = taskData.executeAt.add(taskData.repeatEvery);
    const ixA = await cronos.instruction.frameCreate({
      signer: worker.publicKey,
      timestamp: nextTimestamp,
    });
    const ixB = await cronos.instruction.taskRepeat({
      task: task2PDA.address,
      worker: worker.publicKey,
    });
    await signAndSubmit(provider.connection, [ixA, ixB], worker);

    // Validate next task data.
    const nextTaskPDA = await cronos.account.task.pda(
      daemonPDA.address,
      new BN(2)
    );
    const nextTaskData = await cronos.account.task.data(nextTaskPDA.address);
    assert(nextTaskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(nextTaskData.status)[0] === "pending");
    assert(nextTaskData.executeAt.eq(nextTimestamp));
    assert(nextTaskData.repeatEvery.eq(taskData.repeatEvery));
    assert(nextTaskData.repeatUntil.eq(taskData.repeatUntil));
    assert(nextTaskData.bump === nextTaskPDA.bump);
  });
});

function nextFrameTimestamp(): BN {
  const now = new Date();
  const thisFrame = new Date(now.setSeconds(0, 0));
  const nextFrame = new Date(
    thisFrame.getTime() + ONE_MINUTE.toNumber() * 1000
  );
  return new BN(dateToSeconds(nextFrame));
}
