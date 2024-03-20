use code_explorer::CodeExplorer as CodeExplorerImpl;
use napi_derive::napi;

#[napi(object)]
pub struct ImportBinding {
  pub local: String,
  pub imported: String,
  pub is_type: bool,
}

#[napi(object)]
pub struct ImportResults {
  pub bindings: Vec<ImportBinding>,
  pub source: String,
}

#[napi]
pub struct CodeExplorer {
  inner: CodeExplorerImpl,
}

#[napi]
impl CodeExplorer {
  #[napi(constructor)]
  pub fn new(raw_code: String) -> Self {
    Self {
      inner: CodeExplorerImpl::new(&raw_code),
    }
  }

  #[napi]
  pub fn find_imports(&self) -> Vec<ImportResults> {
    self
      .inner
      .find_imports()
      .into_iter()
      .map(|(source, bds)| ImportResults {
        bindings: bds
          .into_iter()
          .map(|b| ImportBinding {
            local: b.local,
            imported: b.imported,
            is_type: b.is_type,
          })
          .collect(),
        source,
      })
      .collect()
  }
}
