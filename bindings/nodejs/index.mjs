import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import path from "node:path";

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const addon = require("./index.node");

// Re-export the native functions/objects for ESM consumers
const { segment, get_sentence_boundaries } = addon;

export { segment, get_sentence_boundaries };
export default segment;
