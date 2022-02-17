function error(msg: string, error: unknown) {
  console.error(`\n❌ ${msg}`, error);
}

function info(msg: string, ...optionalParams: any[]) {
  console.info("\n", msg, ...optionalParams);
}

function success(msg: string, ...optionalParams: any[]) {
  console.info("\n✅", msg, ...optionalParams);
}

const logger = {
  error,
  info,
  success,
} as const;

export default logger;
