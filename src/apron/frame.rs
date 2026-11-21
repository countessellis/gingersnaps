use serde::{Deserialize,Serialize};
use std::{
  fmt,
};

///////////// ApronFrame

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApronFrame {
  Message {
    id:        String,
    body:      String,
    author:    String,
    timestamp: u64,
  },
  Send {
    body: String,
  },
  Status {
    text: String,
  },
}

impl fmt::Display for ApronFrame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Message { .. } => write!(f,"message"),
      Self::Send    { .. } => write!(f,"send"),
      Self::Status  { .. } => write!(f,"status"),
    }
  }
}

impl ApronFrame {
  pub fn id(&self) -> &str {
    match self {
      Self::Message { id, .. } => id.as_str(),
      _                        => "",
    }
  }
}
