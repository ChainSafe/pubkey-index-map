#![deny(clippy::all)]

use std::collections::hash_map::HashMap;
use napi::{bindgen_prelude::Uint8Array, Error};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct PubkeyIndexMap(HashMap<[u8; 48], i32>);

#[napi]
impl PubkeyIndexMap {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self(HashMap::default())
  }

  #[napi]
  pub fn set(&mut self, key: Uint8Array, value: i32) -> Result<Option<i32>, Error> {
    let res: Result<&[u8; 48], _> = key.as_ref().try_into();
    match res {
      Ok(k) => Ok(self.0.insert(*k, value)),
      Err(_) => Err(Error::from_reason("Input must be 48 bytes")),
    }
  }

  #[napi]
  pub fn get(&self, key: Uint8Array) -> Result<Option<i32>, Error> {
    let res: Result<&[u8; 48], _> = key.as_ref().try_into();
    match res {
      Ok(k) => Ok(self.0.get(k).copied()),
      Err(_) => Err(Error::from_reason("Input must be 48 bytes")),
    }
  }

  #[napi]
  pub fn has(&self, key: Uint8Array) -> Result<bool, Error> {
    let res: Result<&[u8; 48], _> = key.as_ref().try_into();
    match res {
      Ok(k) => Ok(self.0.contains_key(k)),
      Err(_) => Err(Error::from_reason("Input must be 48 bytes")),
    }
  }

  #[napi]
  pub fn delete(&mut self, key: Uint8Array) -> Result<Option<i32>, Error> {
    let res: Result<&[u8; 48], _> = key.as_ref().try_into();
    match res {
      Ok(k) => Ok(self.0.remove(k)),
      Err(_) => Err(Error::from_reason("Input must be 48 bytes")),
    }
  }

  #[napi(getter)]
  pub fn size(&self) -> i32 {
    self.0.len() as i32
  }

  #[napi]
  pub fn clear(&mut self) {
    self.0.clear();
  }

  #[napi]
  pub fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}
