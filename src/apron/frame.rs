use serde::{Deserialize,Serialize};
use std::{
  fmt,
};

use super::*;

///////////// ApronFrame

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(tag = "method", content = "params", rename_all = "snake_case")]
pub enum ApronFrame {
  Auth {
    scheme:    String,
  },
  AuthResult {
    you:       ApronUser,
  },
  Message {
    id:        String,
    body:      String,
    author:    String,
    timestamp: u64,
  },
  Ping,
  Pong,
  Send {
    body:      String,
  },
  Server {
    apron:     u64,
    auth:      Vec<String>,
    ping:      u64,
  },
  Status {
    text:     String,
  },
  #[serde(other)]
  Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WireFrame {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub id: Option<String>,
  #[serde(flatten)]
  pub frame: ApronFrame,
}

impl fmt::Display for ApronFrame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Auth       { .. } => write!(f,"auth"),
      Self::AuthResult { .. } => write!(f,"auth result"),
      Self::Message    { .. } => write!(f,"message"),
      Self::Ping       { .. } => write!(f,"ping"),
      Self::Pong       { .. } => write!(f,"pong"),
      Self::Send       { .. } => write!(f,"send"),
      Self::Server     { .. } => write!(f,"server"),
      Self::Status     { .. } => write!(f,"status"),
      Self::Unknown    { .. } => write!(f,"unknown"),
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
