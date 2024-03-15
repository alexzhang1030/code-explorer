import { defineConfig } from "tsup";
import fg from "fast-glob";
import { basename } from "node:path";
import { copyFileSync } from "node:fs";

export default defineConfig({
  entry: ["./src/index.ts"],
  clean: true,
  dts: true,
  target: "esnext",
  external: [/code-exloprer-binding\..*\.node/, /@code-explorer\/binding-.*/],
  format: ["cjs", "esm"],
  esbuildPlugins: [
    {
      name: "skip-unknown-binary",
      setup(build) {
        build.onResolve({ filter: /.*$/ }, (args) => {
          if (args.path.includes("code-explorer-binding")) console.log("1");
          if (args.kind === "entry-point") return;
          console.log("args", args);
          return {
            path: args.path,
            namespace: "code-explorer-binding",
            external: true,
          };
        });
      },
    },
  ],
  async onSuccess() {
    const files = await fg("./src/code-explorer-binding.*.node", {
      absolute: true,
    });
    // Binary build is on the separate step on CI
    if (!process.env.CI && files.length === 0) {
      throw new Error("No binary files found");
    }
    // Move the binary file to dist
    files.forEach((file) => {
      const fileName = basename(file);
      console.log("Copying", file, "to ./dist");
      copyFileSync(file, `./dist/${fileName}`);
    });
  },
});
