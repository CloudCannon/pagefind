import esbuild from 'esbuild';
import path from 'path';
import ImportGlobPlugin from "esbuild-plugin-import-glob";
import sveltePlugin from "esbuild-svelte";
import { createRequire } from "module";
import { fileURLToPath } from 'url';
import { Server } from 'http';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const { version } = require('./package.json');

const sveltefixPlugin = {
    name: 'fix_svelte_path',
    setup(b) {
        const require = createRequire(import.meta.url);
        const svelteFileLocation = require.resolve('svelte');
        const svelteFolderLocation = path.dirname(svelteFileLocation);
        const nodeFolderLocation = path.dirname(svelteFolderLocation);

        b.onResolve({ filter: /^svelte$|^svelte\// }, args => {
            return { path: path.join(nodeFolderLocation, args.path, 'index.mjs') }
        });
    }
}

const build = async () => {
    const esbuildOptions = {
        write: true,
        watch: false,
        outdir: path.join(__dirname, `../pagefind/vendor/`),
        entryPoints: [path.join(__dirname, 'ui.js')],
        entryNames: `pagefind_[name].${version}`,
        plugins: [
            ImportGlobPlugin.default(),
            sveltePlugin({ compileOptions: { css: false } }),
            sveltefixPlugin
        ],
        minify: true,
        loader: {},
        define: {},
        bundle: true,
    }

    let compiled = await esbuild.build(esbuildOptions);
    console.log(compiled);
}

const serve = async () => {
    const esbuildOptions = {
        outdir: path.join(__dirname, "dev_files/_pagefind"),
        entryPoints: [path.join(__dirname, 'ui.js')],
        plugins: [
            ImportGlobPlugin.default(),
            sveltePlugin({ compileOptions: { css: false } }),
            sveltefixPlugin
        ],
        bundle: true,
    }

    const server = await esbuild.serve({ servedir: path.join(__dirname, "dev_files") }, esbuildOptions);
    console.log(`Serving the dev suite on http://localhost:${server.port}`);
}

if (process.env.PAGEFIND_DEV) {
    serve();
} else {
    build();
}