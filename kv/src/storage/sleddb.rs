use std::{ops::Deref, path::Path};

use prost::Message;
use sled::IVec;

use crate::{KvError, Kvpair, Storage, Value};

pub struct SledTable(sled::Db);

impl SledTable {
    pub fn new(db: sled::Db) -> Self {
        Self(db)
    }
    pub fn open_path(db: impl AsRef<Path>) -> Self {
        let db = sled::open(db).unwrap();
        Self(db)
    }
}

impl Deref for SledTable {
    type Target = sled::Db;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for Kvpair {
    fn from(v: Result<(IVec, IVec), sled::Error>) -> Self {
        match v {
            Ok((k, v)) => match v.as_ref().try_into() {
                Ok(v) => Kvpair::new(String::from_utf8_lossy(k.as_ref()), v),
                Err(_) => Kvpair::default(),
            },
            _ => Kvpair::default(),
        }
    }
}

// convert sled method return to Store method return
fn sled2kv_res(
    result: Result<Option<sled::IVec>, sled::Error>,
) -> Result<Option<crate::Value>, crate::KvError> {
    match result {
        Ok(Some(v)) => match v.as_ref().try_into() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(KvError::ConvertError(Value::from(v).format(), "Value")),
        },
        Ok(None) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

impl Storage for SledTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        let table = self.open_tree(table)?;

        sled2kv_res(table.get(key))
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.open_tree(table)?;

        sled2kv_res(table.insert(key, value.encode_to_vec()))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.open_tree(table)?;

        table.contains_key(key).map_err(|e| e.into())
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.open_tree(table)?;

        sled2kv_res(table.remove(key))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.open_tree(table)?;

        Ok(table.iter().map(|r| r.into()).collect())
    }
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        let table = self.open_tree(table)?;

        Ok(Box::new(table.iter().map(|r| r.into())))
    }
}
