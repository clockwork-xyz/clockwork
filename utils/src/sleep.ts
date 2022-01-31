export async function sleep(time: number) {
  return new Promise((resolve) => setTimeout(resolve, time));
}

export async function sleepUntil(date: Date) {
  const now = new Date();
  const duration = date.getTime() - now.getTime();
  await sleep(duration);
}
