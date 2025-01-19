import esbuild from "esbuild";
import path from "path";
import sveltePlugin from "esbuild-svelte";
import { sveltePreprocess } from "svelte-preprocess";
import { fileURLToPath } from "url";
import fs from "fs";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const js_banner = () => {
  const js_banner_loc = path.join(
    __dirname,
    "../pagefind_web/pkg/pagefind_web.js",
  );
  if (!fs.existsSync(js_banner_loc)) {
    console.error(
      "The pagefind_web.js header cannot be imported from pagefind_web.",
    );
    console.error(
      "Please run the build script in pagefind_web before returning here.",
    );
    process.exit(1);
  }
  return fs.readFileSync(js_banner_loc, { encoding: "utf8" });
};

export const commonOptions = (extraBanner) => {
  return {
    outdir: path.join(__dirname, "output"),
    entryPoints: [
      {
        out: "playground/pagefind-playground",
        in: path.join(__dirname, "lib/playground.ts"),
      },
    ],
    plugins: [
      sveltePlugin({
        compileOptions: { css: "injected" },
        preprocess: sveltePreprocess(),
      }),
    ],
    define: {
      "import.meta.url": '"/pagefind/pagefind.js"',
    },
    banner: {
      js: (extraBanner || "") + js_banner(),
    },
    format: "iife",
    bundle: true,
    mainFields: ["svelte", "browser", "module", "main"],
  };
};
