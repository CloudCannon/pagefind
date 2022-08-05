const fs = require("fs");
const path = require("path");

const changelogFile = path.join(__dirname, "../CHANGELOG.md");

const err = (m) => {
    console.error(m);
    process.exit(1);
}

if (!fs.existsSync(changelogFile)) err(`Script expected a file at ${changelogFile}`);

let contents = fs.readFileSync(changelogFile, { encoding: "utf-8" });
let version = "", lines = contents.split(/\n/g);
let it = lines.entries();

while (!(entry = it.next()).done) {
    let [, line] = entry.value;
    // Read until we reach the section for the latest version.
    if (/^\s*##\s+v/i.test(line)) {
        version = line.match(/^\s*##\s+(v\S+)/)[1];
        if (version) {
            console.log(version);
            process.exit(0);
        }
    }
}

err("No version found in changelog");
