const os = require('os');
const fs = require('fs');
const path = require('path');
const util = require('util');
const child_process = require('child_process');

const download = require('./download');

const fsExists = util.promisify(fs.exists);
const mkdir = util.promisify(fs.mkdir);
const exec = util.promisify(child_process.exec);

const VERSION = `v${require('../package.json').version}`;
const BIN_PATH = path.join(__dirname, '../bin');

process.on('unhandledRejection', (reason, promise) => {
    console.log('Unhandled rejection: ', promise, 'reason:', reason);
});

async function isMusl() {
    let stderr;
    try {
        stderr = (await exec('ldd --version')).stderr;
    } catch (err) {
        stderr = err.stderr;
    }
    if (stderr.indexOf('musl') > -1) {
        return true;
    }
    return false;
}

async function getTarget() {
    const arch = process.env.npm_config_arch || os.arch();

    switch (os.platform()) {
        case 'darwin':
            return 'x86_64-apple-darwin';
            // TODO: Waiting for GitHub actions to build on M1.
            return arch === 'arm64' ? 'aarch64-apple-darwin' :
                'x86_64-apple-darwin';
        case 'win32':
            return arch === 'x64' ? 'x86_64-pc-windows-msvc' :
                arch === 'arm' ? 'aarch64-pc-windows-msvc' :
                    'i686-pc-windows-msvc';
        case 'linux':
            return arch === 'x64' ? 'x86_64-unknown-linux-musl' :
                arch === 'arm' ? 'arm-unknown-linux-gnueabihf' :
                    arch === 'armv7l' ? 'arm-unknown-linux-gnueabihf' :
                        arch === 'arm64' ? await isMusl() ? 'aarch64-unknown-linux-musl' : 'aarch64-unknown-linux-gnu' :
                            arch === 'ppc64' ? 'powerpc64le-unknown-linux-gnu' :
                                arch === 's390x' ? 's390x-unknown-linux-gnu' :
                                    'i686-unknown-linux-musl'
        default: throw new Error('Unsupported platform: ' + os.platform());
    }
}

async function main() {
    const binExists = await fsExists(BIN_PATH);

    if (!binExists) {
        await mkdir(BIN_PATH);
    }

    const opts = {
        version: VERSION,
        target: await getTarget(),
        destDir: BIN_PATH
    };
    try {
        await download(opts);
    } catch (err) {
        console.error(`Downloading pagefind failed: ${err.stack}`);
        process.exit(1);
    }
}

main();
