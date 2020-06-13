//! String interning.

use std::collections::HashMap;

/// A reference to a string. To learn what string this represents, you must ask the StrStore created
/// from the StrStoreMut that returned this StrRef to you.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StrRef(usize);

// TODO generate this with a macro?
impl StrRef {
  pub const STAR: StrRef = StrRef(0);
  pub const INT: StrRef = StrRef(1);
  pub const REAL: StrRef = StrRef(2);
  pub const WORD: StrRef = StrRef(3);
  pub const CHAR: StrRef = StrRef(4);
  pub const STRING: StrRef = StrRef(5);
  pub const LIST: StrRef = StrRef(6);
  pub const NIL: StrRef = StrRef(7);
  pub const CONS: StrRef = StrRef(8);
  pub const TRUE: StrRef = StrRef(9);
  pub const FALSE: StrRef = StrRef(10);
}

/// A mutable factory of StrRefs. Allows creating new StrRefs from Strings.
pub struct StrStoreMut {
  store: HashMap<String, StrRef>,
  next: usize,
}

impl StrStoreMut {
  /// Returns an new StrStoreMut containing only the special StrRefs.
  pub fn new() -> Self {
    let mut store = HashMap::with_capacity(11);
    store.insert("*".to_owned(), StrRef::STAR);
    store.insert("int".to_owned(), StrRef::INT);
    store.insert("real".to_owned(), StrRef::REAL);
    store.insert("word".to_owned(), StrRef::WORD);
    store.insert("char".to_owned(), StrRef::CHAR);
    store.insert("string".to_owned(), StrRef::STRING);
    store.insert("list".to_owned(), StrRef::LIST);
    store.insert("nil".to_owned(), StrRef::NIL);
    store.insert("::".to_owned(), StrRef::CONS);
    store.insert("true".to_owned(), StrRef::TRUE);
    store.insert("false".to_owned(), StrRef::FALSE);
    Self {
      next: store.len(),
      store,
    }
  }

  /// Inserts a string slice into this StrStoreMut. Returns an StrRef corresponding to that string.
  /// Converts the string slice to an owned String iff this string slice was not already present.
  pub fn insert_str(&mut self, s: &str) -> StrRef {
    if let Some(&id) = self.store.get(s) {
      return id;
    }
    let ret = StrRef(self.next);
    self.store.insert(s.to_owned(), ret);
    self.next += 1;
    ret
  }

  /// Same as insert, but better if you already have ownership of the string.
  pub fn insert_string(&mut self, s: String) -> StrRef {
    if let Some(&id) = self.store.get(&s) {
      return id;
    }
    let ret = StrRef(self.next);
    self.store.insert(s, ret);
    self.next += 1;
    ret
  }

  /// Converts this StrStoreMut into an StrStore, preventing further mutation.
  pub fn finish(self) -> StrStore {
    let mut store = vec![String::new(); self.store.len()];
    for (s, id) in self.store {
      // each index should be assigned exactly once, based on the way we handed out StrRefs.
      store[id.0] = s;
    }
    StrStore { store }
  }
}

/// An immutable store of Strings. Allows looking up the String corresponding to a StrRef.
pub struct StrStore {
  store: Vec<String>,
}

impl StrStore {
  /// Returns the string slice corresponding to this StrRef.
  pub fn get(&self, id: StrRef) -> &str {
    self.store[id.0].as_str()
  }
}
