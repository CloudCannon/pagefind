const fs = require("fs");
const path = require("path");

const version = process.env.GIT_VERSION;
if (!version) {
    console.error("Script expected a GIT_VERSION environment variable");
    process.exit(1);
}

const pkg = path.join(__dirname, "../wrappers/node/package.json");

const pkg_contents = JSON.parse(fs.readFileSync(pkg, { encoding: "utf-8" }));
for (const dep of Object.keys(pkg_contents.optionalDependencies).filter(dep => dep.startsWith("@pagefind"))) {
    pkg_contents.optionalDependencies[dep] = version;
}

console.log(JSON.stringify(pkg_contents.optionalDependencies, null, 2));

fs.writeFileSync(pkg, JSON.stringify(pkg_contents, null, 2));
