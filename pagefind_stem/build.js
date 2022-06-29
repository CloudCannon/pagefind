/**
 * A rough build script for generating this module out of the snowball repo.
 * - Expects a `snowball` repo alongside the `pagefind` repo.
 * - Expects `make` to have been run in the `snowball` repo, such that the `snowball` binary exists
 * - Generates all files in snowball/*
 */

const fs = require("fs");
const path = require("path");
const cp = require("child_process");

const snowballDir = path.join(__dirname, "../../snowball/");
const snowballBinary = path.join(snowballDir, "./snowball");
const thisSnowballDir = path.join(__dirname, "./src/snowball/");
const algorithmDir = path.join(snowballDir, "algorithms/");

const algorithms = fs.readdirSync(algorithmDir).filter(a => a.endsWith(".sbl")).map(a => {
    let name = a.split('.')[0];
    return { file: a, low: name, high: name.replace(/^(\w)/, c => c.toUpperCase()) };
});

const rustTemplate = (algorithms) => {
    return `
${algorithms.map(a => `#[cfg(feature = "${a.low}")]\npub mod ${a.low};`).join('\n')}

pub enum Algorithm {
${algorithms.map(a => `    #[cfg(feature = "${a.low}")]\n    ${a.high},`).join('\n')}
}

impl From<Algorithm> for fn(&mut super::SnowballEnv) -> bool {
    fn from(lang: Algorithm) -> Self {
        match lang {
${algorithms.map(a => `            #[cfg(feature = "${a.low}")]\n            Algorithm::${a.high} => ${a.low}::stem,`).join('\n')}
        }
    }
}`;
}

fs.rmSync(thisSnowballDir, { recursive: true, force: true });
fs.mkdirSync(thisSnowballDir);
fs.cpSync(path.join(snowballDir, "rust/src/snowball"), thisSnowballDir, { recursive: true });
fs.writeFileSync(path.join(__dirname, "./src/snowball/algorithms/mod.rs"), rustTemplate(algorithms));

algorithms.forEach(algorithm => {
    cp.execSync(`${snowballBinary} ${path.join(algorithmDir, algorithm.file)} -o src/snowball/algorithms/${algorithm.low} -rust`, {
        stdio: "inherit",
        cwd: __dirname
    });
})

console.log(algorithms.map(a => `${a.low} = []`).join('\n'));
console.log(`---\n^ These features need to be added to Cargo.toml`);