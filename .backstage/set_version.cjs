const fs = require("fs");
const path = require("path");

const changelogFile = path.join(__dirname, "../CHANGELOG.md");
const docsNavFile = path.join(__dirname, "../docs/data/nav.yml");

const err = (m) => {
  console.error(m);
  process.exit(1);
};

if (!fs.existsSync(changelogFile))
  err(`Script expected a file at ${changelogFile}`);

let contents = fs.readFileSync(changelogFile, { encoding: "utf-8" });
let version = process.env.HUGO_PAGEFIND_DOCS_VERSION || "",
  lines = contents.split(/\n/g);
let it = lines.entries();

if (!version) {
  while (!(entry = it.next()).done) {
    let [, line] = entry.value;
    // Read until we reach the section for the latest version.
    if (/^\s*##\s+v/i.test(line)) {
      version = line.match(/^\s*##\s+(v\S+)/)[1];
      if (version) {
        break;
      }
    }
  }
}

if (!version) {
  err("No version found in changelog");
}

let docsNavContents = fs.readFileSync(docsNavFile, { encoding: "utf-8" });
docsNavContents = docsNavContents.replace(
  /\blink_label:.*Local.*$/gim,
  `link_label: "${version}"`
);
fs.writeFileSync(docsNavFile, docsNavContents);

console.log(version);
