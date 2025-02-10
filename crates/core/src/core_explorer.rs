mod find_imports;

pub use find_imports::{find_imports, ImportBinding};
use std::collections::HashMap;
use tree_sitter::{Parser, Tree};

pub struct CodeExplorer {
  raw_code: String,
  tree: Tree,
}

impl CodeExplorer {
  pub fn new(raw_code: &str) -> CodeExplorer {
    // Initialize the parser
    let mut parser = Parser::new();
    let language = tree_sitter_typescript::language_typescript();
    parser
      .set_language(language)
      .expect("Error loading TypeScript language");

    // Parse the code
    let tree = parser.parse(raw_code, None).expect("Error parsing code");

    CodeExplorer {
      raw_code: raw_code.to_string(),
      tree,
    }
  }

  pub fn find_imports(&self) -> HashMap<String, Vec<ImportBinding>> {
    find_imports(&self.raw_code, &self.tree)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let code = r#"
        import { a } from 'b';
        import { c } from 'd';
        "#;
    let res = CodeExplorer::new(&code.to_string()).find_imports();
    assert_eq!(res.len(), 2);
  }
}
