#!/usr/bin/env node
const os = require('os');
const { spawnSync } = require('child_process');

const execname = 'pagefind';

function resolveBinaryPath() {
    const cpu = process.env.npm_config_arch || os.arch();
    const platform = process.platform === 'win32' ? 'windows' : process.platform;

    const executable = platform === 'windows' ? `${execname}.exe` : execname;

    try {
        return require.resolve(`@${execname}/${platform}-${cpu}/bin/${executable}`);
    } catch (e) {
        console.error(`Failed to install ${execname}. Most likely the platform ${platform}-${cpu} is not yet a supported architecture.`);
        console.error(`Please open an issue at https://github.com/CloudCannon/${execname} and paste this error message in full.`);
        console.error(`If you believe this package should be compatible with your system,`)
        console.error(`you can try downloading a release binary directly from https://github.com/CloudCannon/${execname}/releases`);
        process.exit(1);
    }
}

try {
    const args = process.argv.slice(2);
    const binaryPath = resolveBinaryPath();
    const verbose = args.filter(a => /verbose|-v$/i.test(a)).length;
    if (verbose) {
        console.log(`${execname} npm wrapper: Running the executable at ${binaryPath}`);
    }
    const processResult = spawnSync(binaryPath, args, {
        windowsHide: true,
        stdio: [process.stdin, process.stdout, process.stderr]
    });
    if (verbose) {
        console.log(`${execname} npm wrapper: Process exited with status ${processResult.status}`);
    }
    process.exit(processResult.status ?? 1);
} catch (err) {
    console.error(`Failed to run ${execname} via the npx wrapper: ${err}`);
    console.error(`Please open an issue at https://github.com/CloudCannon/${execname} and paste this error message in full.`);
    process.exit(1);
}
