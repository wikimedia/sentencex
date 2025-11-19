import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const { platform, arch } = process;
const packageName = `sentencex-${platform}-${arch}`;

const require = createRequire(import.meta.url);

// Re-export the native functions/objects for ESM consumers
const { segment, get_sentence_boundaries } = require(packageName);

export { segment, get_sentence_boundaries };
export default segment;
