import esbuild from "esbuild";
import path from "path";
import { fileURLToPath } from "url";
import { commonOptions } from "./_build_common.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const serve = async () => {
  const esbuildOptions = {
    ...commonOptions("const pagefind_version = 'PLAYGROUND';"),
    conditions: ["svelte", "browser", "development"],
  };

  const context = await esbuild.context(esbuildOptions);
  const server = await context.serve({
    servedir: path.join(__dirname, "output"),
  });
  console.log(
    `Serving the playground http://localhost:${server.port}/playground/`,
  );
};

serve();
