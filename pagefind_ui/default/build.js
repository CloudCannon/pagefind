import esbuild from "esbuild";
import path from "path";
import ImportGlobPlugin from "esbuild-plugin-import-glob";
import sveltePlugin from "esbuild-svelte";
import { createRequire } from "module";
import { fileURLToPath } from "url";
import fs from "fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const { version } = require("./package.json");

const sveltefixPlugin = {
  name: "fix_svelte_path",
  setup(b) {
    const require = createRequire(import.meta.url);
    const svelteFileLocation = require.resolve("svelte");
    const svelteFolderLocation = path.dirname(svelteFileLocation);
    const nodeFolderLocation = path.dirname(svelteFolderLocation);

    b.onResolve({ filter: /^svelte$|^svelte\// }, (args) => {
      return { path: path.join(nodeFolderLocation, args.path, "index.mjs") };
    });
  },
};

const serve = async () => {
  const esbuildOptions = {
    outdir: path.join(__dirname, "_dev_files/pagefind"),
    entryPoints: [
      path.join(__dirname, "ui.js"),
      {
        out: "pagefind",
        in: path.join(__dirname, "_dev_files/pagefind/_pagefind_stub.ts"),
      },
    ],
    plugins: [
      ImportGlobPlugin.default(),
      sveltePlugin({ compileOptions: { css: false } }),
      sveltefixPlugin,
    ],
    format: "esm",
    bundle: true,
  };

  const context = await esbuild.context(esbuildOptions);
  const server = await context.serve({
    servedir: path.join(__dirname, "_dev_files"),
  });
  console.log(`Serving the dev suite on http://localhost:${server.port}`);
};

const build = async () => {
  const commonOpts = {
    write: true,
    plugins: [
      ImportGlobPlugin.default(),
      sveltePlugin({ compileOptions: { css: false } }),
      sveltefixPlugin,
    ],
    loader: {},
    define: {},
    bundle: true,
  };

  // Direct web vendor build
  const esbuildVendorOptions = {
    ...commonOpts,
    minify: true,
    entryPoints: [path.join(__dirname, "ui.js")],
    entryNames: `pagefind_[name].${version}`,
    outdir: path.join(__dirname, `../../pagefind/vendor/`),
  };
  const compiledVendor = await esbuild.build(esbuildVendorOptions);
  console.log(`Vendor Build: `, compiledVendor);

  // CJS "main" build
  const esbuildCjsOptions = {
    ...commonOpts,
    entryPoints: [path.join(__dirname, "ui-core.js")],
    outdir: path.join(__dirname, `npm_dist/cjs/`),
    outExtension: { ".js": ".cjs" },
    platform: "node",
  };
  const compiledCJS = await esbuild.build(esbuildCjsOptions);
  stripCSSComment(path.join(__dirname, `npm_dist/cjs/ui-core.css`));
  console.log(`CJS Build: `, compiledCJS);

  // ESM Module Build
  const esbuildModuleOptions = {
    ...commonOpts,
    entryPoints: [path.join(__dirname, "ui-core.js")],
    outdir: path.join(__dirname, `npm_dist/mjs/`),
    outExtension: { ".js": ".mjs" },
    platform: "neutral",
  };
  const compiledMJS = await esbuild.build(esbuildModuleOptions);
  stripCSSComment(path.join(__dirname, `npm_dist/mjs/ui-core.css`));
  console.log(`Module Build: `, compiledMJS);

  try {
    fs.mkdirSync(path.join(__dirname, `css`));
  } catch {}
  fs.copyFileSync(
    path.join(__dirname, `npm_dist/mjs/ui-core.css`),
    path.join(__dirname, `css/ui.css`)
  );
};

const stripCSSComment = (file) => {
  let contents = fs.readFileSync(file, { encoding: "utf-8" });
  contents = contents.replace(/\/\* fakecss[^*]+\*\//g, "");
  fs.writeFileSync(file, contents);
};

if (process.env.PAGEFIND_DEV) {
  serve();
} else {
  build();
}
