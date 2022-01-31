import { Provider } from "@project-serum/anchor";
import { AccountLayout, NATIVE_MINT, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, SystemProgram, Transaction } from "@solana/web3.js";
import {getOrCreateATA} from "./ata";

export type WrapSolProps = {
  lamports: number;
  provider: Provider;
};

export async function wrapSol({ lamports, provider }: WrapSolProps) {
  // Get connection and wallet
  const connection = provider.connection;
  const wallet = provider.wallet;

  // Start a new transaction
  const transaction = new Transaction();
  transaction.feePayer = wallet.publicKey;
  const blockhashObj = await connection.getRecentBlockhash();
  transaction.recentBlockhash = await blockhashObj.blockhash;

  // Get the users wSOL associated token account.
  // If the account doesn't exist, add an instruction to create it.
  const { address, instruction } = await getOrCreateATA({
    provider: provider,
    mint: NATIVE_MINT,
    owner: wallet.publicKey,
    payer: wallet.publicKey
  });
  if (instruction) transaction.add(instruction);

  // Add instructions to transaction
  const newAccount = Keypair.generate();
  const balanceNeeded = await Token.getMinBalanceRentForExemptAccount(connection);
  transaction.add(
    // Create a new account.
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: newAccount.publicKey,
      lamports: balanceNeeded,
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID
    }),

    // Send lamports to the new account.
    SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: newAccount.publicKey,
      lamports: lamports
    }),

    // Assign the new account to the native token mint.
    // This will be wrap the lamports into wSOL.
    Token.createInitAccountInstruction(
      TOKEN_PROGRAM_ID,
      NATIVE_MINT,
      newAccount.publicKey,
      wallet.publicKey
    ),

    // Transfer tokens from the new account to the user's wSOL associated token account.
    Token.createTransferInstruction(
      TOKEN_PROGRAM_ID,
      newAccount.publicKey,
      address,
      wallet.publicKey,
      [newAccount],
      lamports
    ),

    // Close the new account and return the rent to the user.
    Token.createCloseAccountInstruction(
      TOKEN_PROGRAM_ID,
      newAccount.publicKey,
      wallet.publicKey,
      wallet.publicKey,
      [newAccount]
    )
  );

  // Sign the transaction
  transaction.partialSign(newAccount);
  const signedTransaction = await wallet.signTransaction(transaction);

  // Send and confirm the signed transaction
  const sig = await connection.sendRawTransaction(signedTransaction.serialize());
  await connection.confirmTransaction(sig);
}
