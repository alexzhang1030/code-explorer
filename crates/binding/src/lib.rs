use code_explorer::find_imports as find_imports_impl;
use napi_derive::napi;

#[napi(object)]
pub struct ImportResults {
  pub ids: Vec<String>,
  pub source: String,
}

#[napi]
pub fn find_imports(code: String) -> Vec<ImportResults> {
  let res = find_imports_impl(&code);
  res
    .into_iter()
    .map(|(source, ids)| ImportResults {
      ids: ids.ids,
      source,
    })
    .collect()
}
