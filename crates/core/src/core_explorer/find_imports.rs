use std::collections::HashMap;
use tree_sitter::{Query, QueryCursor, Tree};

const QUERY: &'static str = r"
(import_statement
  (import_clause
    (named_imports
      (import_specifier
        (identifier) @import.id
      )
    )
  )
  source: (string) @import.source
)";

#[derive(Debug, PartialEq)]
pub struct ImportStmt {
  pub ids: Vec<String>,
}

pub fn find_imports(raw_code: &String, tree: &Tree) -> HashMap<String, ImportStmt> {
  let root_node = tree.root_node();

  // Prepare the query
  let query = Query::new(tree.language(), QUERY).expect("Error compiling query");
  let mut query_cursor = QueryCursor::new();

  // Find the matches
  let code_bytes = raw_code.as_bytes();
  let captures = query_cursor.matches(&query, root_node, code_bytes);
  let mut map: HashMap<String, ImportStmt> = HashMap::new();
  let capture_names = &query.capture_names();

  // Process the matches
  for match_ in captures {
    for (index, c) in match_.captures.iter().enumerate() {
      let capture_name = &capture_names[c.index as usize];
      if capture_name == "import.id" {
        continue;
      }
      let previous = match_.captures.get(index - 1);
      if let Some(previous) = previous {
        if capture_names[previous.index as usize] == "import.source" {
          continue;
        }
        let code = c.node.utf8_text(code_bytes).unwrap().to_string();
        let previous_code = previous.node.utf8_text(code_bytes).unwrap().to_string();
        map
          .entry(code)
          .or_insert(ImportStmt { ids: vec![] })
          .ids
          .push(previous_code);
      }
    }
  }
  map
}

#[cfg(test)]
mod tests {
  use super::find_imports;
  use tree_sitter::Parser;

  #[test]
  fn find_imports_works() {
    let code = r#"
        import { a } from 'b';
        import { c } from 'd';
        "#;

    // Initialize the parser
    let mut parser = Parser::new();
    let language = tree_sitter_typescript::language_typescript();
    parser
      .set_language(language)
      .expect("Error loading TypeScript language");

    // Parse the code
    let tree = parser.parse(code, None).expect("Error parsing code");

    let res = find_imports(&code.to_string(), &tree);
    assert_eq!(res.len(), 2);
  }
}
