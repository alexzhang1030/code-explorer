import { findImports } from 'code-explorer'

it('find imports', () => {
  const code = `
    import { foo } from 'bar'
  `
  const res = findImports(code)
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
