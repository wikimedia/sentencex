const { platform, arch } = process;
const packageName = `sentencex-${platform}-${arch}`;

try {
  module.exports = require(packageName);
} catch (error) {
  if (error.code === "MODULE_NOT_FOUND") {
    throw new Error(
      `No prebuilt binary found for ${packageName}. ` +
        `Please install build tools and rebuild from source.`,
    );
  }
  throw error;
}
