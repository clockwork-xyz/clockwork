export default function invariant(condition: any, message?: string): asserts condition {
  if (!condition) throw new Error(message);
}
