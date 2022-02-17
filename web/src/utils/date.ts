import { BN } from "@project-serum/anchor";

export function dateToSeconds(date: Date): number {
  return Math.floor(date.getTime() / 1000);
}

export function now(): BN {
  return new BN(dateToSeconds(new Date()));
}
