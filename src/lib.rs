#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use std::collections::hash_map::HashMap;

#[macro_use]
extern crate napi_derive;

type Val = i32;

#[napi]
pub struct PubkeyIndexMap(HashMap<[u8; 48], Val>);

#[napi]
impl PubkeyIndexMap {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self(HashMap::default())
  }

  #[napi]
  pub fn set(&mut self, key: Uint8Array, value: Val) -> Result<Option<Val>> {
    Ok(self.0.insert(*to_arr(&key)?, value))
  }

  #[napi]
  pub fn get(&self, key: Uint8Array) -> Result<Option<Val>> {
    Ok(self.0.get(to_arr(&key)?).copied())
  }

  #[napi]
  pub fn has(&self, key: Uint8Array) -> Result<bool> {
    Ok(self.0.contains_key(to_arr(&key)?))
  }

  #[napi]
  pub fn delete(&mut self, key: Uint8Array) -> Result<Option<Val>> {
    Ok(self.0.remove(to_arr(&key)?))
  }

  #[napi(getter)]
  pub fn size(&self) -> Val {
    self.0.len() as Val
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

fn to_arr<'a>(a: &'a Uint8Array) -> Result<&'a [u8; 48]> {
  a
    .as_ref()
    .try_into()
    .map_err(|_| Error::from_reason("Input must be 48 bytes"))
}
