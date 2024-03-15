import { defineBuildConfig } from "unbuild";
import nodeFs from "node:fs";
import nodePath from "node:path";
import fg from "fast-glob";

export default defineBuildConfig({
  entries: ["./src/index"],
  clean: true,
  declaration: true,
  externals: [/code-explorer-binding\..*\.node$/, /@code-explorer\/binding-.*/],
  rollup: {
    emitCJS: true,
    cjsBridge: true,
    inlineDependencies: true,
  },
  hooks: {
    "build:done"(_ctx) {
      console.log("triggered");
      const binaryFiles = fg.sync("./src/code-explorer-binding.*.node", {
        absolute: true,
      });
      // Binary build is on the separate step on CI
      if (!process.env.CI && binaryFiles.length === 0) {
        throw new Error("No binary files found");
      }
      // Move the binary file to dist
      binaryFiles.forEach((file) => {
        const fileName = nodePath.basename(file);
        console.log("Copying", file, "to ./dist");
        nodeFs.copyFileSync(file, `./dist/${fileName}`);
      });
    },
  },
});
