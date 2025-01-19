import esbuild from "esbuild";
import path from "path";
import fs from "fs";
import { fileURLToPath } from "url";
import { commonOptions } from "./_build_common.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const build = async () => {
  const esbuildOptions = {
    ...commonOptions(""),
    write: true,
    minify: true,
    conditions: ["svelte", "browser", "production"],
  };
  const compiled = await esbuild.build(esbuildOptions);
  console.log(`Build: `, compiled);

  const vendorDir = path.join(__dirname, `../pagefind/vendor/`);
  try {
    fs.mkdirSync(vendorDir);
  } catch {}

  fs.cpSync(
    path.join(__dirname, `output/playground`),
    path.join(vendorDir, `playground`),
    { recursive: true },
  );
};

build();
