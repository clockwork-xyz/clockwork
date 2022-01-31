import { Provider } from "@project-serum/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { TransactionInstruction } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";

type Result = {
  /**
   * ATA key
   */
  address: PublicKey;
  /**
   * Instruction to create the account if it doesn't exist.
   */
  instruction: TransactionInstruction | null;
};

/**
 * Gets an associated token account, returning a create instruction if it doesn't exist.
 * @param param0
 * @returns
 */
export const getOrCreateATA = async ({
  provider,
  mint,
  owner = provider.wallet.publicKey,
  payer = provider.wallet.publicKey
}: {
  provider: Provider;
  mint: PublicKey;
  owner?: PublicKey;
  payer?: PublicKey;
}): Promise<Result> => {
  const address = await getATAAddress({ mint, owner });
  if (await provider.connection.getAccountInfo(address)) {
    return { address, instruction: null };
  } else {
    return {
      address,
      instruction: createATAInstruction({
        mint,
        address,
        owner,
        payer
      })
    };
  }
};

/**
 * Instruction for creating an ATA.
 * @returns
 */
export const createATAInstruction = ({
  address,
  mint,
  owner,
  payer
}: {
  address: PublicKey;
  mint: PublicKey;
  owner: PublicKey;
  payer: PublicKey;
}): TransactionInstruction =>
  Token.createAssociatedTokenAccountInstruction(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint,
    address,
    owner,
    payer
  );

/**
 * Gets an associated token account address.
 */
export const getATAAddress = async ({
  mint,
  owner
}: {
  mint: PublicKey;
  owner: PublicKey;
}): Promise<PublicKey> => {
  const [address] = await PublicKey.findProgramAddress(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
  return address;
};
