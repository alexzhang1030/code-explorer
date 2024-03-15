import nodeFs from 'node:fs'
import nodePath from 'node:path'
import { env } from 'node:process'
import { defineBuildConfig } from 'unbuild'
import fg from 'fast-glob'

export default defineBuildConfig({
  entries: ['./src/index'],
  clean: true,
  declaration: true,
  externals: [/code-explorer-binding\..*\.node$/, /@code-explorer\/binding-.*/],
  rollup: {
    emitCJS: true,
    cjsBridge: true,
    inlineDependencies: true,
  },
  hooks: {
    'build:done': function (_ctx) {
      const binaryFiles = fg.sync('./src/code-explorer-binding.*.node', {
        absolute: true,
      })
      // Binary build is on the separate step on CI
      if (!env.CI && binaryFiles.length === 0)
        throw new Error('No binary files found')

      // Move the binary file to dist
      binaryFiles.forEach((file) => {
        const fileName = nodePath.basename(file)
        // eslint-disable-next-line no-console
        console.log('Copying', file, 'to ./dist')
        nodeFs.copyFileSync(file, `./dist/${fileName}`)
      })
    },
  },
})
