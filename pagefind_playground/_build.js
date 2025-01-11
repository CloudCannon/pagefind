import esbuild from "esbuild";
import { commonOptions } from "./_build_common.js";

const build = async () => {
  const esbuildOptions = {
    ...commonOptions(""),
    write: true,
    minify: true,
    conditions: ["svelte", "browser", "production"],
  };
  const compiled = await esbuild.build(esbuildOptions);
  console.log(`Build: `, compiled);

  // try {
  //   fs.mkdirSync(path.join(__dirname, `css`));
  // } catch {}
  // fs.copyFileSync(
  //   path.join(__dirname, `npm_dist/mjs/ui-core.css`),
  //   path.join(__dirname, `css/ui.css`),
  // );
};

build();
