const path = require('path');
const fs = require('fs');
const os = require('os');
const https = require('https');
const util = require('util');
const url = require('url');
const URL = url.URL;
const child_process = require('child_process');
const proxy_from_env = require('proxy-from-env');
const { createHash } = require('crypto');

const packageVersion = require('../package.json').version;
const cacheDir = path.join(__dirname, `../cache`);
const tmpDir = path.join(cacheDir, packageVersion);

const fsUnlink = util.promisify(fs.unlink);
const fsExists = util.promisify(fs.exists);
const fsMkdir = util.promisify(fs.mkdir);

const isWindows = os.platform() === 'win32';

const REPO = 'CloudCannon/pagefind';

function downloadWin(url, dest, opts) {
    return new Promise((resolve, reject) => {
        let userAgent;
        if (opts.headers['user-agent']) {
            userAgent = opts.headers['user-agent'];
            delete opts.headers['user-agent'];
        }
        const headerValues = Object.keys(opts.headers)
            .map(key => `\\"${key}\\"=\\"${opts.headers[key]}\\"`)
            .join('; ');
        const headers = `@{${headerValues}}`;
        console.log('Downloading with Invoke-WebRequest');
        dest = sanitizePathForPowershell(dest);
        let iwrCmd = `[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest -URI ${url} -UseBasicParsing -OutFile ${dest} -Headers ${headers}`;
        if (userAgent) {
            iwrCmd += ' -UserAgent ' + userAgent;
        }
        if (opts.proxy) {
            iwrCmd += ' -Proxy ' + opts.proxy;

            try {
                const { username, password } = new URL(opts.proxy);
                if (username && password) {
                    const decodedPassword = decodeURIComponent(password);
                    iwrCmd += ` -ProxyCredential (New-Object PSCredential ('${username}', (ConvertTo-SecureString '${decodedPassword}' -AsPlainText -Force)))`;
                }
            } catch (err) {
                reject(err);
            }
        }

        iwrCmd = `powershell -noprofile "${iwrCmd}"`;

        child_process.exec(iwrCmd, err => {
            if (err) {
                reject(err);
                return;
            }
            resolve();
        });
    });
}

function download(_url, dest, opts) {

    const proxy = proxy_from_env.getProxyForUrl(url.parse(_url));
    if (proxy !== '') {
        var HttpsProxyAgent = require('https-proxy-agent');
        opts = {
            ...opts,
            "agent": new HttpsProxyAgent(proxy),
            proxy
        };
    }

    if (isWindows) {
        // This alternative strategy shouldn't be necessary but sometimes on Windows the file does not get closed,
        // so unzipping it fails, and I don't know why.
        return downloadWin(_url, dest, opts);
    }

    return new Promise((resolve, reject) => {
        console.log(`Download options: ${JSON.stringify(opts)}`);
        const outFile = fs.createWriteStream(dest);
        const mergedOpts = {
            ...url.parse(_url),
            ...opts
        };
        https.get(mergedOpts, response => {
            console.log('statusCode: ' + response.statusCode);
            if (response.statusCode === 302) {
                console.log('Following redirect to: ' + response.headers.location);
                return download(response.headers.location, dest, opts)
                    .then(resolve, reject);
            } else if (response.statusCode !== 200) {
                reject(new Error('Download failed with ' + response.statusCode));
                return;
            }

            response.pipe(outFile);
            outFile.on('finish', () => {
                resolve();
            });
        }).on('error', async err => {
            await fsUnlink(dest);
            reject(err);
        });
    });
}

function get(_url, opts) {
    console.log(`GET ${_url}`);

    const proxy = proxy_from_env.getProxyForUrl(url.parse(_url));
    if (proxy !== '') {
        var HttpsProxyAgent = require('https-proxy-agent');
        opts = {
            ...opts,
            "agent": new HttpsProxyAgent(proxy)
        };
    }

    return new Promise((resolve, reject) => {
        let result = '';
        opts = {
            ...url.parse(_url),
            ...opts
        };
        https.get(opts, response => {
            if (response.statusCode !== 200) {
                reject(new Error('Request failed: ' + response.statusCode));
            }

            response.on('data', d => {
                result += d.toString();
            });

            response.on('end', () => {
                resolve(result);
            });

            response.on('error', e => {
                reject(e);
            });
        });
    });
}

function getApiUrl(repo, tag) {
    if (/v\d+\.\d+\.\d+-.+$/.test(tag)) {
        console.log(`https://api.github.com/repos/${repo}-beta/releases/tags/${tag}`)
        return `https://api.github.com/repos/${repo}-beta/releases/tags/${tag}`;
    } else {
        console.log(`https://api.github.com/repos/${repo}/releases/tags/${tag}`)
        return `https://api.github.com/repos/${repo}/releases/tags/${tag}`;
    }
}

/**
 * @param {{ force: boolean; token: string; version: string; }} opts
 * @param {string} assetName
 * @param {string} downloadFolder
 */
async function getAssetFromGithubApi(opts, assetName, downloadFolder) {
    const assetDownloadPath = path.join(downloadFolder, assetName);

    // We can just use the cached binary
    if (await fsExists(assetDownloadPath)) {
        console.log('Using cached download: ' + assetDownloadPath);
        return assetDownloadPath;
    }

    const downloadOpts = {
        headers: {
            'user-agent': 'pagefind'
        }
    };

    console.log(`Finding release for ${opts.version}`);
    const release = await get(getApiUrl(REPO, opts.version), downloadOpts);
    let jsonRelease;
    try {
        jsonRelease = JSON.parse(release);
    } catch (e) {
        throw new Error('Malformed API response: ' + e.stack);
    }

    if (!jsonRelease.assets) {
        throw new Error('Bad API response: ' + JSON.stringify(release));
    }

    const asset = jsonRelease.assets.find(a => a.name === assetName);
    if (!asset) {
        throw new Error('Asset not found with name: ' + assetName);
    }

    console.log(`Downloading from ${asset.url}`);
    console.log(`Downloading to ${assetDownloadPath}`);

    downloadOpts.headers.accept = 'application/octet-stream';
    await download(asset.url, assetDownloadPath, downloadOpts);
}

// Handle whitespace in filepath as powershell split's path with whitespaces
function sanitizePathForPowershell(path) {
    path = path.replace(/ /g, '` '); // replace whitespace with "` " as solution provided here https://stackoverflow.com/a/18537344/7374562
    return path;
}

function untar(zipPath, destinationDir) {
    return new Promise((resolve, reject) => {
        const unzipProc = child_process.spawn('tar', ['xvf', zipPath, '-C', destinationDir], { stdio: 'inherit' });
        unzipProc.on('error', err => {
            reject(err);
        });
        unzipProc.on('close', code => {
            console.log(`tar xvf exited with ${code}`);
            if (code !== 0) {
                reject(new Error(`tar xvf exited with ${code}`));
                return;
            }

            resolve();
        });
    });
}

async function unzipPagefind(zipPath, destinationDir) {
    const expectedName = path.join(destinationDir, 'pagefind_extended');

    if (await fsExists(expectedName)) {
        await fsUnlink(expectedName);
    }

    if (await fsExists(expectedName + '.exe')) {
        await fsUnlink(expectedName + '.exe');
    }

    await untar(zipPath, destinationDir);

    if (await fsExists(expectedName)) {
        return expectedName;
    }

    if (await fsExists(expectedName + '.exe')) {
        return expectedName + '.exe';
    }

    throw new Error(`Expecting pagefind_extended or pagefind_extended.exe unzipped into ${destinationDir}, didn't find one.`);
}

async function cleanCache() {
    return Promise.all(fs.readdirSync(cacheDir).map((dir) => {
        if (dir != packageVersion) {
            return fs.promises.rm(path.join(cacheDir, dir), { recursive: true })
        }
    }))
}

function verifyChecksum(assetName, downloadFolder) {
    const checksumPath = path.join(__dirname, '../checksums', `${assetName}.sha256`);
    const releaseSum = fs.readFileSync(checksumPath, 'utf8').split(" ")[0];

    const assetDownloadPath = path.join(downloadFolder, assetName);
    const assetBuffer = fs.readFileSync(assetDownloadPath);
    const hash = createHash('sha256');
    hash.update(assetBuffer);
    const assetSum = hash.digest("hex");

    if (assetSum !== releaseSum) {
        throw new Error("Integrity check failed.")
    }
}

module.exports = async opts => {
    if (!opts.version) {
        return Promise.reject(new Error('Missing version'));
    }

    if (!opts.target) {
        return Promise.reject(new Error('Missing target'));
    }

    const assetName = ['pagefind_extended', opts.version, opts.target].join('-') + '.tar.gz';

    if (!await fsExists(tmpDir)) {
        await fsMkdir(tmpDir, { recursive: true });
    }

    const assetDownloadPath = path.join(tmpDir, assetName);
    try {
        await getAssetFromGithubApi(opts, assetName, tmpDir)
        await verifyChecksum(assetName, tmpDir);
    } catch (e) {
        console.log('Deleting invalid download cache');
        try {
            await fsUnlink(assetDownloadPath);
            const expectedName = path.join(opts.destDir, 'pagefind_extended');

            if (await fsExists(expectedName)) {
                await fsUnlink(expectedName);
            }

            if (await fsExists(expectedName + '.exe')) {
                await fsUnlink(expectedName + '.exe');
            }
        } catch (e) { }

        throw e;
    }

    console.log(`Unzipping to ${opts.destDir}`);
    try {
        const destinationPath = await unzipPagefind(assetDownloadPath, opts.destDir);
        if (!isWindows) {
            await util.promisify(fs.chmod)(destinationPath, '755');
        }
    } catch (e) {
        console.log('Deleting invalid download');

        try {
            await fsUnlink(assetDownloadPath);
        } catch (e) { }

        throw e;
    }

    await cleanCache()
};
