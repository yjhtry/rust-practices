#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn query(sql: String, output: String) -> String {
  let rt = tokio::runtime::Runtime::new().unwrap();
  let mut data = rt.block_on(async { queryer::query(sql).await.unwrap() });

  match output.as_str() {
    "csv" => data.to_csv().unwrap(),
    v => format!("Output type {} not supported", v),
  }
}
