import { CodeExplorer } from 'code-explorer'

it('find imports with CodeExplorer', () => {
  const code = `
    import { a, b as bb, type c, type d as dd } from 's1'
    import type { o, p as pp, type q, type r as rr } from 's2'
  `
  const res = new CodeExplorer(code).findImports()
  expect(res).toMatchInlineSnapshot(`
    [
      {
        "bindings": [
          {
            "imported": "a",
            "isType": false,
            "local": "a",
          },
          {
            "imported": "b",
            "isType": false,
            "local": "bb",
          },
          {
            "imported": "c",
            "isType": true,
            "local": "c",
          },
          {
            "imported": "d",
            "isType": true,
            "local": "dd",
          },
        ],
        "source": "'s1'",
      },
      {
        "bindings": [
          {
            "imported": "o",
            "isType": true,
            "local": "o",
          },
          {
            "imported": "p",
            "isType": true,
            "local": "pp",
          },
          {
            "imported": "q",
            "isType": true,
            "local": "q",
          },
          {
            "imported": "r",
            "isType": true,
            "local": "rr",
          },
        ],
        "source": "'s2'",
      },
    ]
  `)
})
