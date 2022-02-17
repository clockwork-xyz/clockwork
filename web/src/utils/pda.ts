import { PublicKey } from "@solana/web3.js";

// PDA
export type PDA = {
  address: PublicKey;
  bump: number;
};

export async function findPDA(
  seeds: Array<Buffer | Uint8Array>,
  programId: PublicKey
): Promise<PDA> {
  const [indexAddress, indexBump] = await PublicKey.findProgramAddress(
    seeds,
    programId
  );
  return {
    address: indexAddress,
    bump: indexBump,
  };
}
