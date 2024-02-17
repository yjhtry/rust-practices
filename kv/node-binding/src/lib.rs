#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[macro_use]
extern crate lazy_static;

use kv::{CommandRequest, MemTable, Service, Value};
use napi::bindgen_prelude::Either5;

lazy_static! {
  static ref SVC: Service = Service::new(MemTable::new());
}

#[napi]
pub fn hget(table: String, key: String) -> serde_json::Value {
  let cmd = CommandRequest::new_hget(table, key);
  let res = SVC.execute(cmd);

  serde_json::json!(res)
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
  let res = SVC.execute(cmd);

  serde_json::json!(res)
}

#[napi]
pub fn h_get_all(table: String) -> serde_json::Value {
  let cmd = CommandRequest::new_hgetall(table);
  let res = SVC.execute(cmd);

  serde_json::json!(res)
}
