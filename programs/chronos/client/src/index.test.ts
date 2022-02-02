import assert from "assert";
import {
  dateToSeconds,
  newSigner,
  PDA,
  signAndSubmit,
  sleepUntil,
} from "@chronos-so/utils";
import { BN, Provider, setProvider } from "@project-serum/anchor";
import { Token, TOKEN_PROGRAM_ID, NATIVE_MINT } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Chronos } from "./";

const ONE_MINUTE = new BN(60);

describe("Chronos", () => {
  // Configure the client to use the local cluster.
  const provider = Provider.env();
  setProvider(provider);

  let chronos = new Chronos(provider);

  let signer: Keypair;
  let worker: Keypair;
  let authorityPDA: PDA;
  let daemonPDA: PDA;
  let frame0PDA: PDA;
  let taskA0PDA: PDA;
  let taskB0PDA: PDA;
  let taskB1PDA: PDA;
  let signerTokens: PublicKey;
  let daemonTokens: PublicKey;
  let tokenProgram: Token;
  let timestamp: BN;
  let TRANSFER_AMOUNT = new BN(0.05 * LAMPORTS_PER_SOL);

  before(async () => {
    signer = await newSigner(provider.connection);
    worker = await newSigner(provider.connection);
    daemonPDA = await chronos.account.daemon.pda(signer.publicKey);
    tokenProgram = new Token(
      provider.connection,
      NATIVE_MINT,
      TOKEN_PROGRAM_ID,
      signer
    );
  });

  it("Initializes", async () => {
    // Submit instruction.
    const ix = await chronos.instruction.initialize({
      signer: signer.publicKey,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate authority account.
    authorityPDA = await chronos.account.authority.pda();
    const authorityData = await chronos.account.authority.data(
      authorityPDA.address
    );
    assert(authorityData.bump == authorityPDA.bump);

    // TODO Validate config account.
    // TODO Validate treasury account.
  });

  it("Creates a daemon", async () => {
    // Submit instruction.
    let ix = await chronos.instruction.daemonCreate({
      owner: signer.publicKey,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate daemon account.
    let daemonData = await chronos.account.daemon.data(daemonPDA.address);
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
    let transferIx = Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      daemonTokens,
      signerTokens,
      daemonPDA.address,
      [],
      TRANSFER_AMOUNT.toNumber()
    );
    const ix = await chronos.instruction.daemonInvoke({
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
    const ix = await chronos.instruction.frameCreate({
      signer: signer.publicKey,
      timestamp: timestamp,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate frame account.
    frame0PDA = await chronos.account.frame.pda(timestamp);
    const frameData = await chronos.account.frame.data(frame0PDA.address);
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
    const ix = await chronos.instruction.taskCreate({
      daemon: daemonPDA.address,
      executeAt: timestamp,
      repeatEvery: new BN(0),
      repeatUntil: timestamp,
      instruction: transferIx,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate task data.
    taskA0PDA = await chronos.account.task.pda(daemonPDA.address, new BN(0));
    let taskData = await chronos.account.task.data(taskA0PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.executeAt.eq(timestamp));
    assert(taskData.repeatEvery.eq(new BN(0)));
    assert(taskData.repeatUntil.eq(timestamp));
    assert(taskData.bump === taskA0PDA.bump);
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
    const ix = await chronos.instruction.taskCreate({
      daemon: daemonPDA.address,
      executeAt: timestamp,
      repeatEvery: ONE_MINUTE,
      repeatUntil: timestamp.add(ONE_MINUTE),
      instruction: transferIx,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate task data.
    taskB0PDA = await chronos.account.task.pda(daemonPDA.address, new BN(1));
    let taskData = await chronos.account.task.data(taskB0PDA.address);
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

    // Sumbit instruction.
    const ix = await chronos.instruction.taskExecute({
      task: taskA0PDA.address,
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
    const taskData = await chronos.account.task.data(taskA0PDA.address);
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
    const ix = await chronos.instruction.taskExecute({
      task: taskB0PDA.address,
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
    const taskData = await chronos.account.task.data(taskB0PDA.address);
    assert(Object.keys(taskData.status)[0] === "repeat");
  });

  it("Repeats a recurring task", async () => {
    // Submit instructions
    const next_timestamp = timestamp.add(ONE_MINUTE);
    const ixA = await chronos.instruction.frameCreate({
      signer: worker.publicKey,
      timestamp: next_timestamp,
    });
    const ixB = await chronos.instruction.taskRepeat({
      task: taskB0PDA.address,
      worker: worker.publicKey,
    });
    await signAndSubmit(provider.connection, [ixA, ixB], worker);

    // TODO validate task data.
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
