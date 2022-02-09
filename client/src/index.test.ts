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
    timestamp = new BN(dateToSeconds(new Date()) + 10);
    const ix = await cronos.instruction.taskCreate({
      daemon: daemonPDA.address,
      execAt: timestamp,
      stopAt: timestamp,
      recurr: new BN(0),
      instruction: transferIx,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate task data.
    task1PDA = await cronos.account.task.pda(daemonPDA.address, new BN(0));
    let taskData = await cronos.account.task.data(task1PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.execAt.eq(timestamp));
    assert(taskData.stopAt.eq(timestamp));
    assert(taskData.recurr.eq(new BN(0)));
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
      execAt: timestamp,
      stopAt: timestamp.add(ONE_MINUTE),
      recurr: ONE_MINUTE,
      instruction: transferIx,
    });
    await signAndSubmit(provider.connection, [ix], signer);

    // Validate task data.
    task2PDA = await cronos.account.task.pda(daemonPDA.address, new BN(1));
    let taskData = await cronos.account.task.data(task2PDA.address);
    assert(taskData.daemon.toString() === daemonPDA.address.toString());
    assert(Object.keys(taskData.status)[0] === "pending");
    assert(taskData.execAt.eq(timestamp));
    assert(taskData.stopAt.eq(timestamp.add(ONE_MINUTE)));
    assert(taskData.recurr.eq(ONE_MINUTE));
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
    const preTaskData = await cronos.account.task.data(task2PDA.address);
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
    assert(taskData.execAt.eq(preTaskData.execAt.add(preTaskData.recurr)));
    assert(Object.keys(taskData.status)[0] === "pending");
  });
});
