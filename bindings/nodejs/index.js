import { createRequire } from "module";
const require = createRequire(import.meta.url);

const { segment } = require("./index.node");

export { segment };
