const { segment } = require('../bindings/nodejs');

function main() {
    const languageCode = "en";
    const text = "Hello world. This is a test.";
    const sentences = segment(languageCode, text);

    console.log("Segmented sentences:", sentences);
}

main();
