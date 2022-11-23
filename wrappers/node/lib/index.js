#!/usr/bin/env node
const path = require('path');
const fs = require('fs');
const { execFileSync } = require('child_process');

const binaryPath = path.join(__dirname, `../bin/pagefind_extended${process.platform === 'win32' ? '.exe' : ''}`);

if (!fs.existsSync(binaryPath)) {
    console.error(`Binary failed to download — package expected to have downloaded the binary to ${binaryPath} during its postinstall`);
    console.error(`Do you have install scripts disabled for npm?`);
    console.error(`Try running \`npm config set ignore-scripts false\` and re-installing.`);
    console.error(`Otherwise, your platform might not be supported. Open an issue on GitHub!`);
    process.exit(1);
}

try {
    execFileSync(
        binaryPath,
        process.argv.slice(2),
        {
            stdio: [process.stdin, process.stdout, process.stderr]
        }
    )
} catch (err) {
    process.exit(1);
}