import esbuild from "esbuild";
import path from "path";
import { createRequire } from "module";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const { version } = require("./package.json");

const build = async () => {
  const commonOpts = {
    write: true,
    plugins: [],
    loader: {},
    define: {},
    bundle: true,
  };

  // Coupled search vendor build
  const esbuildVendorOptions = {
    ...commonOpts,
    entryPoints: [path.join(__dirname, "lib/public_search_api.ts")],
    entryNames: `pagefind_[name].${version}`,
    outdir: path.join(__dirname, `../pagefind/vendor/`),
    format: "esm",
    target: "es2020",
  };
  const compiledVendor = await esbuild.build(esbuildVendorOptions);
  console.log(`Vendor Build: `, compiledVendor);

  // Coupled highlight vendor build
  const esbuildVendorHighlightOptions = {
    ...commonOpts,
    entryPoints: [path.join(__dirname, "lib/highlight.ts")],
    entryNames: `pagefind_[name].${version}`,
    outdir: path.join(__dirname, `../pagefind/vendor/`),
    format: "esm",
    target: "es2020",
  };
  const compiledVendorHighlight = await esbuild.build(
    esbuildVendorHighlightOptions,
  );
  console.log(`Vendor Highlight Build: `, compiledVendorHighlight);
};

build();
