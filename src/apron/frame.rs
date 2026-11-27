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
  Me {
    name:      Option<String>,
    avatar:    Option<String>,
  },
  Message {
    id:        ApronId,
    body:      String,
    author:    String,
    timestamp: u64,
  },
  Ping {},
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

impl fmt::Display for ApronFrame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Auth       { .. } => write!(f,"auth"),
      Self::AuthResult { .. } => write!(f,"auth result"),
      Self::Me         { .. } => write!(f,"me"),
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
      Self::Message { id, .. } => id,
      _                        => "",
    }
  }
}

///////////// WireFrame

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WireFrame {
  #[serde(default)]
  pub id:     Option<String>,
  pub method: String,
  #[serde(default)]
  pub params:  serde_json::Value,
}

impl WireFrame {
  pub fn into_frame(self) -> ApronFrame {
    let params = if self.params.is_null() {
      serde_json::json!({})
    } else {
      self.params
    };
    serde_json::from_value(serde_json::json!({ "method": self.method, "params": params })).unwrap_or(ApronFrame::Unknown)
  }
}

///////////// ReplyFrame

#[derive(Serialize)]
pub struct ReplyFrame<'a,T: serde::Serialize> {
  pub id: &'a str,
  pub result: T,
}

