import { CodeExplorer } from 'code-explorer'

it('find imports with CodeExplorer', () => {
  const code = `
    import { foo } from 'bar'
  `
  const res = new CodeExplorer(code).findImports()
  expect(res).toMatchInlineSnapshot(`
    [
      {
        "ids": [
          "foo",
        ],
        "source": "'bar'",
      },
    ]
  `)
})
