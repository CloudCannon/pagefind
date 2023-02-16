const fs = require("fs");
const path = require("path");

const web_js_file = path.join(__dirname, "pkg/pagefind_web.js");
let web_js = fs.readFileSync(web_js_file, { encoding: "utf-8" });

// document.currentScript.src breaks in the module context
// and is never a code path we need, so we nix that from the
// generated javascript.
web_js = web_js.replace(/document\..?currentScript\..?src/, `"UNHANDLED"`);

fs.writeFileSync(web_js_file, web_js);
