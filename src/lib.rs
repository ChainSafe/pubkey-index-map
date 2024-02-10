#![deny(clippy::all)]

use std::collections::hash_map::HashMap;
use napi::bindgen_prelude::Uint8Array;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct PubkeyIndexMap {
  map: HashMap<Uint8ArrayWrapper, i32>,
}

// Uint8ArrayWrapper is a wrapper around Uint8Array to make it hashable
#[derive(Clone)]
struct Uint8ArrayWrapper {
  key: Uint8Array,
}

impl Eq for Uint8ArrayWrapper {}

impl PartialEq for Uint8ArrayWrapper {
  fn eq(&self, other: &Self) -> bool {
    self.key.as_ref().eq(other.key.as_ref())
  }
}

impl std::hash::Hash for Uint8ArrayWrapper {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.key.as_ref().hash(state);
  }
}

#[napi]
impl PubkeyIndexMap {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      map: std::collections::HashMap::new(),
    }
  }

  #[napi(factory)]
  pub fn with_initial_capacity(capacity: i32) -> Self {
    Self {
      map: std::collections::HashMap::with_capacity(capacity as usize),
    }
  }

  #[napi]
  pub fn set(&mut self, key: Uint8Array, value: i32) {
    self.map.insert(Uint8ArrayWrapper { key }, value);
  }

  #[napi]
  pub fn get(&self, key: Uint8Array) -> Option<i32> {
    self.map.get(&Uint8ArrayWrapper { key }).copied()
  }

  #[napi]
  pub fn has(&self, key: Uint8Array) -> bool {
    self.map.contains_key(&Uint8ArrayWrapper { key })
  }

  #[napi]
  pub fn delete(&mut self, key: Uint8Array) -> Option<i32> {
    self.map.remove(&Uint8ArrayWrapper { key })
  }

  #[napi(getter)]
  pub fn size(&self) -> i32 {
    self.map.len() as i32
  }

  #[napi]
  pub fn clear(&mut self) {
    self.map.clear();
  }

  #[napi]
  pub fn clone(&self) -> Self {
    Self {
      map: self.map.clone(),
    }
  }
}
