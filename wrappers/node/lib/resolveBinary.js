import os from 'os';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);

/**
 * @param {string} execname 
 * @returns {string}
 */
export function resolveBinaryPath(execname) {
    const env_var = process.env[`${execname.toUpperCase()}_BINARY_PATH`];
    if (env_var) return env_var;

    const cpu = process.env.npm_config_arch || os.arch();
    const platform = process.platform === 'win32' ? 'windows' : process.platform;

    const executable = platform === 'windows' ? `${execname}.exe` : execname;

    try {
        return require.resolve(`@${execname}/${platform}-${cpu}/bin/${executable}`);
    } catch (e) {
        throw new Error(
            [
                `Failed to install ${execname}. Most likely the platform ${platform}-${cpu} is not yet a supported architecture.`,
                `Please open an issue at https://github.com/CloudCannon/${execname} and paste this error message in full.`,
                `If you believe this package should be compatible with your system,`,
                `you can try downloading a release binary directly from https://github.com/CloudCannon/${execname}/releases`
            ].join('\n')
        );
    }
}