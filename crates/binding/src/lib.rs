use code_explorer::CodeExplorer as CodeExplorerImpl;
use napi_derive::napi;

#[napi(object)]
pub struct ImportResults {
  pub ids: Vec<String>,
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
      .map(|(source, ids)| ImportResults {
        ids: ids.ids,
        source,
      })
      .collect()
  }
}
