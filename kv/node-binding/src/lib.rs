#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate lazy_static;

use futures::StreamExt;
use kv::{CommandRequest, MemTable, Service, Value};
use napi::bindgen_prelude::Either5;

lazy_static! {
  static ref SVC: Service = Service::new(MemTable::new());
}

#[napi]
pub fn hget(table: String, key: String) -> serde_json::Value {
  let cmd = CommandRequest::new_hget(table, key);
  let mut res = SVC.execute(cmd);

  let data = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(async { res.next().await.unwrap() });

  serde_json::json!(*data)
}

#[napi]
pub fn hset(
  table: String,
  key: String,
  value: Either5<String, i64, f64, bool, Vec<u8>>,
) -> serde_json::Value {
  let value: Value = match value {
    Either5::A(a) => a.into(),
    Either5::B(b) => b.into(),
    Either5::C(c) => c.into(),
    Either5::D(d) => d.into(),
    Either5::E(e) => e.into(),
  };

  let cmd = CommandRequest::new_hset(table, key, value);
  let mut res = SVC.execute(cmd);

  let data = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(async { res.next().await.unwrap() });

  serde_json::json!(*data)
}

#[napi]
pub fn h_get_all(table: String) -> serde_json::Value {
  let cmd = CommandRequest::new_hgetall(table);
  let mut res = SVC.execute(cmd);

  let data = tokio::runtime::Runtime::new()
    .unwrap()
    .block_on(async { res.next().await.unwrap() });

  serde_json::json!(*data)
}
