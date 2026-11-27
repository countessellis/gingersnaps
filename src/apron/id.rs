use chrono::Utc;
use serde::{Deserialize,Serialize};
use std::{
  borrow::Borrow,
  fmt,
  ops::Deref,
};

///////////// ApronId

#[derive(Serialize,Deserialize,Clone,Debug,Eq,Hash,PartialEq)]
#[serde(transparent)]
pub struct ApronId {
  pub id: String,
}

impl fmt::Display for ApronId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,"{}",self.id)
  }
}

impl AsRef<str> for ApronId {
  fn as_ref(&self) -> &str {
    &self.id
  }
}

impl Borrow<str> for ApronId {
  fn borrow(&self) -> &str {
    &self.id
  }
}

impl Deref for ApronId {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    &self.id
  }
}

impl From<String> for ApronId {
  fn from(id: String) -> Self {
    Self { id }
  }
}

impl From<&str> for ApronId {
  fn from(id: &str) -> Self {
    Self { id: id.to_owned() }
  }
}

impl ApronId {
  pub fn new(id: impl Into<Self>) -> Self {
    id.into()
  }

  pub fn from_now() -> Self {
    Self {
      id: (Utc::now().timestamp_millis() as u64).to_string(),
    }
  }
}
