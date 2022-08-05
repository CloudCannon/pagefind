const fs = require("fs");
const path = require("path");

const version = process.env.GIT_VERSION;
const version_re = /version\s*[=:]\s*"0.0.0"/;

const err = (m) => {
    console.error(m);
    process.exit(1);
}

if (!version) err("Script expected a GIT_VERSION environment variable");

const file = (localPath) => {
    localPath = path.join(__dirname, localPath);
    if (!fs.existsSync(localPath)) err(`Script expected a file at ${localPath}`);
    const contents = fs.readFileSync(localPath, { encoding: "utf-8" });
    if (!version_re.test(contents)) err(`Expected ${localPath} to contain a version of "0.0.0"`);
    return { path: localPath, contents };
}

let pagefindCfg = file("../pagefind/Cargo.toml");
pagefindCfg.contents = pagefindCfg.contents.replace(version_re, `version = "${version}"`);
fs.writeFileSync(pagefindCfg.path, pagefindCfg.contents);

let pagefindWebCfg = file("../pagefind_web/Cargo.toml");
pagefindWebCfg.contents = pagefindWebCfg.contents.replace(version_re, `version = "${version}"`);
fs.writeFileSync(pagefindWebCfg.path, pagefindWebCfg.contents);

let pagefindUiCfg = file("../pagefind_ui/package.json");
pagefindUiCfg.contents = pagefindUiCfg.contents.replace(version_re, `version: "${version}"`);
fs.writeFileSync(pagefindUiCfg.path, pagefindUiCfg.contents);
