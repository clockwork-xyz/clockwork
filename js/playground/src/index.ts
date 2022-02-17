import App from "./app";
import logger from "./logger";

require("dotenv").config();

new App()
  .start()
  .then(() => logger.success("started app"))
  .catch((error) => logger.error("failed to start app", error));
