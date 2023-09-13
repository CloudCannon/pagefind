import esbuild from 'esbuild';
import path from 'path';
import { createRequire } from "module";
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const { version } = require('./package.json');

const build = async () => {
    const commonOpts = {
        write: true,
        plugins: [],
        loader: {},
        define: {},
        bundle: true,
    }

    // Coupled search vendor build
    const esbuildVendorOptions = {
        ...commonOpts,
        entryPoints: [path.join(__dirname, 'lib/coupled_search.ts')],
        entryNames: `pagefind_[name].${version}`,
        outdir: path.join(__dirname, `../pagefind/vendor/`),
        format: 'esm',
        target: 'es2020'
    }
    const compiledVendor = await esbuild.build(esbuildVendorOptions);
    console.log(`Vendor Build: `, compiledVendor);

    // // CJS "main" build
    // const esbuildCjsOptions = {
    //     ...commonOpts,
    //     entryPoints: [path.join(__dirname, 'ui-core.js')],
    //     outdir: path.join(__dirname, `npm_dist/cjs/`),
    //     outExtension: { '.js': '.cjs' },
    //     platform: 'node',
    // }
    // const compiledCJS = await esbuild.build(esbuildCjsOptions);
    // console.log(`CJS Build: `, compiledCJS);

    // // ESM Module Build
    // const esbuildModuleOptions = {
    //     ...commonOpts,
    //     entryPoints: [path.join(__dirname, 'ui-core.js')],
    //     outdir: path.join(__dirname, `npm_dist/mjs/`),
    //     outExtension: { '.js': '.mjs' },
    //     platform: 'neutral',
    // }
    // const compiledMJS = await esbuild.build(esbuildModuleOptions);
    // console.log(`Module Build: `, compiledMJS);
}

build();