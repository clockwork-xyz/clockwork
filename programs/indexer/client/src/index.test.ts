import assert from "assert";
import { BN, Provider, setProvider } from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";
import { airdrop, PDA, signAndSubmit } from "@chronos-so/utils";
import { Indexer } from "../src";

describe("Indexer", () => {
  // Configure the listProgram to use the local cluster.
  const provider = Provider.env();
  setProvider(provider);
  const listProgram = new Indexer(provider);

  // Shared test data.
  const addressA = Keypair.generate().publicKey;
  const addressB = Keypair.generate().publicKey;
  const namespace = Keypair.generate().publicKey;
  const owner = Keypair.generate();
  const signer = Keypair.generate();
  let listPDA: PDA;

  before(async () => {
    await airdrop(1, owner.publicKey, provider.connection);
    await airdrop(1, signer.publicKey, provider.connection);
  });

  it("creates a list", async () => {
    // Generate instruction and accounts.
    const ix = await listProgram.instruction.createList({
      owner: owner.publicKey,
      payer: owner.publicKey,
      namespace: namespace,
    });

    // Sign and submit transaction.
    await signAndSubmit(provider.connection, [ix], owner);

    // Validate index account state.
    listPDA = await listProgram.account.list.pda(owner.publicKey, namespace);
    const listData = await listProgram.account.list.data(listPDA.address);
    assert.ok(listData.owner.toString() === owner.publicKey.toString());
    assert.ok(listData.namespace.toString() === namespace.toString());
    assert.ok(listData.count.toNumber() === 0);
    assert.ok(listData.bump === listPDA.bump);
  });

  it("pushes an element to index 0", async () => {
    // Generate instruction.
    const name = "foo";
    const ix = await listProgram.instruction.pushElement({
      list: listPDA.address,
      owner: owner.publicKey,
      value: addressA,
    });

    // Sign and submit transaction.
    await signAndSubmit(provider.connection, [ix], owner);

    // Validate index account data.
    const listData = await listProgram.account.list.data(listPDA.address);
    assert.ok(listData.owner.toString() === owner.publicKey.toString());
    assert.ok(listData.namespace.toString() === namespace.toString());
    assert.ok(listData.count.toNumber() === 1);
    assert.ok(listData.bump === listPDA.bump);

    // Validate pointer account data.
    const ZERO = new BN(0);
    const elementPDA = await listProgram.account.element.pda(
      listPDA.address,
      ZERO
    );
    const elementData = await listProgram.account.element.data(
      elementPDA.address
    );
    assert.ok(elementData.index.eq(ZERO));
    assert.ok(elementData.value.toString() === addressA.toString());
    assert.ok(elementData.bump === elementPDA.bump);
  });

  it("pushes an element to index 1", async () => {
    // Generate instruction.
    const ix = await listProgram.instruction.pushElement({
      list: listPDA.address,
      owner: owner.publicKey,
      value: addressB,
    });

    // Sign and submit transaction.
    await signAndSubmit(provider.connection, [ix], owner);

    // Validate index account data.
    const listData = await listProgram.account.list.data(listPDA.address);
    assert.ok(listData.owner.toString() === owner.publicKey.toString());
    assert.ok(listData.namespace.toString() === namespace.toString());
    assert.ok(listData.count.toNumber() === 2);
    assert.ok(listData.bump === listPDA.bump);

    // Validate pointer account data.
    const ONE = new BN(1);
    const elementPDA = await listProgram.account.element.pda(
      listPDA.address,
      ONE
    );
    const elementData = await listProgram.account.element.data(
      elementPDA.address
    );
    assert.ok(elementData.index.eq(ONE));
    assert.ok(elementData.value.toString() === addressB.toString());
    assert.ok(elementData.bump === elementPDA.bump);
  });

  it("pops the element at index 1", async () => {
    // Generate instruction.
    const ix = await listProgram.instruction.popElement({
      list: listPDA.address,
    });

    // Sign and submit transaction.
    await signAndSubmit(provider.connection, [ix], owner);

    // Validate pointer data.
    const ONE = new BN(1);
    const elementPDA = await listProgram.account.element.pda(
      listPDA.address,
      ONE
    );
    await assert.rejects(
      listProgram.account.element.data(elementPDA.address),
      `Error: Account does not exist ${elementPDA.address}`
    );

    // Validate index data.
    const listData = await listProgram.account.list.data(listPDA.address);
    assert.ok(listData.owner.toString() === owner.publicKey.toString());
    assert.ok(listData.namespace.toString() === namespace.toString());
    assert.ok(listData.count.toNumber() === 1);
    assert.ok(listData.bump === listPDA.bump);
  });

  it("pops the element at index 0", async () => {
    // Generate instruction.
    const ix = await listProgram.instruction.popElement({
      list: listPDA.address,
    });

    // Sign and submit transaction.
    await signAndSubmit(provider.connection, [ix], owner);

    // Validate pointer data.
    const ZERO = new BN(0);
    const elementPDA = await listProgram.account.element.pda(
      listPDA.address,
      ZERO
    );
    await assert.rejects(
      listProgram.account.element.data(elementPDA.address),
      `Error: Account does not exist ${elementPDA.address}`
    );

    // Validate index data.
    const listData = await listProgram.account.list.data(listPDA.address);
    assert.ok(listData.owner.toString() === owner.publicKey.toString());
    assert.ok(listData.namespace.toString() === namespace.toString());
    assert.ok(listData.count.toNumber() === 0);
    assert.ok(listData.bump === listPDA.bump);
  });

  it("deletes a list", async () => {
    // Generate instruction.
    const ix = await listProgram.instruction.deleteList({
      list: listPDA.address,
    });

    // Sign and submit transaction.
    await signAndSubmit(provider.connection, [ix], owner);

    // Validate pointer data.
    await assert.rejects(
      listProgram.account.list.data(listPDA.address),
      `Error: Account does not exist ${listPDA.address}`
    );
  });
});
