use std::collections::HashMap;
use tree_sitter::{Query, QueryCursor, Tree};

const QUERY: &'static str = r"
(import_statement
  (import_clause
    (named_imports
      (import_specifier
        name: (identifier) @import.imported
        alias: (identifier)? @import.alias
      ) @import.specifier
    )
  )
  source: (string) @import.source
) @import.statment";

#[derive(Debug, PartialEq)]
pub struct ImportBinding {
  pub local: String,
  pub imported: String,
  pub is_type: bool,
}

pub fn find_imports(raw_code: &String, tree: &Tree) -> HashMap<String, Vec<ImportBinding>> {
  let root_node = tree.root_node();

  // Prepare the query
  let query = Query::new(tree.language(), QUERY).expect("Error compiling query");
  let mut query_cursor = QueryCursor::new();

  // Find the matches
  let code_bytes = raw_code.as_bytes();
  let captures = query_cursor.matches(&query, root_node, code_bytes);
  let mut map: HashMap<String, Vec<ImportBinding>> = HashMap::new();
  let capture_names = &query.capture_names();

  // Process the matches
  for match_ in captures {
    let mut source = String::new();
    let mut local = String::new();
    let mut imported = String::new();
    let mut is_type = false;

    for c in match_.captures.iter() {
      let capture_name = &capture_names[c.index as usize];
      let capture_text = c.node.utf8_text(code_bytes).unwrap().to_string();

      match capture_name.as_str() {
        "import.source" => source = capture_text,
        "import.imported" => imported = capture_text,
        "import.alias" => local = capture_text,
        "import.specifier" => {
          if capture_text.starts_with("type ") && is_type == false {
            is_type = true
          }
        }
        "import.statment" => {
          if capture_text.starts_with("import type ") && is_type == false {
            is_type = true
          }
        }
        _ => {}
      }
    }

    if !imported.is_empty() {
      let binding = ImportBinding {
        local: match local.as_str() {
          "" => imported.clone(),
          _ => local.clone(),
        },
        imported: imported.clone(),
        is_type,
      };
      map
        .entry(source.clone())
        .or_insert_with(Vec::new)
        .push(binding);
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
        import type { a, b as bb, type c, type d as dd } from 'source1';
        import { a, b as bb, type c, type d as dd } from 'source2';
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
