import esbuild from 'esbuild';
import path from 'path';
import ImportGlobPlugin from "esbuild-plugin-import-glob";
import fs from 'fs';

import { createRequire } from "module";
import { fileURLToPath } from 'url';


const __dirname = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const { version } = require('./package.json');


const serve = async () => {
    const esbuildOptions = {
        outdir: path.join(__dirname, "_dev_files/_pagefind"),
        entryPoints: [path.join(__dirname, 'modular.js')],
        plugins: [
            ImportGlobPlugin.default(),
        ],
        bundle: true,
    }
    fs.copyFileSync(path.join(__dirname, `css/ui.css`), path.join(__dirname, `_dev_files/_pagefind/modular.css`));

    const context = await esbuild.context(esbuildOptions);
    const server = await context.serve({ servedir: path.join(__dirname, "_dev_files") });
    console.log(`Serving the dev suite on http://localhost:${server.port}`);
}

const build = async () => {
    const commonOpts = {
        write: true,
        plugins: [
            ImportGlobPlugin.default(),
        ],
        loader: {},
        define: {},
        bundle: true,
    }

    // Direct web vendor build
    const esbuildVendorOptions = {
        ...commonOpts,
        outdir: path.join(__dirname, `../../pagefind/vendor/`),
        entryPoints: [path.join(__dirname, 'modular.js')],
        entryNames: `pagefind_[name]_ui.${version}`,
        minify: true,
    }
    const compiledVendor = await esbuild.build(esbuildVendorOptions);
    console.log(compiledVendor);

    // CJS "main" build
    const esbuildCjsOptions = {
        ...commonOpts,
        entryPoints: [path.join(__dirname, 'modular-core.js')],
        outdir: path.join(__dirname, `npm_dist/cjs/`),
        outExtension: { '.js': '.cjs' },
        platform: 'node',
    }
    const compiledCJS = await esbuild.build(esbuildCjsOptions);
    console.log(`CJS Build: `, compiledCJS);

    // ESM Module Build
    const esbuildModuleOptions = {
        ...commonOpts,
        entryPoints: [path.join(__dirname, 'modular-core.js')],
        outdir: path.join(__dirname, `npm_dist/mjs/`),
        outExtension: { '.js': '.mjs' },
        platform: 'neutral',
    }
    const compiledMJS = await esbuild.build(esbuildModuleOptions);
    console.log(`Module Build: `, compiledMJS);

    fs.copyFileSync(path.join(__dirname, `css/ui.css`), path.join(__dirname, `../../pagefind/vendor/pagefind_modular_ui.${version}.css`));
}

if (process.env.PAGEFIND_DEV) {
    serve();
} else {
    build();
}