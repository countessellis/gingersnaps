use crossbeam_channel::Sender;

use crate::apron::*;

///////////// ServerRequest

#[derive(Debug, Clone)]
pub struct ServerRequest {
  pub command: String,
  pub args:    ServerArgs,
  pub sender:  Sender<ServerResponse>,
}

///////////// ServerArgs

#[derive(Debug, Clone)]
pub enum ServerArgs {
  ChatFrame(ApronFrame),
  Id(u64),
  Text(String),
  None,
}

///////////// ServerResponse

#[derive(Debug, Clone)]
pub enum ServerResponse {
  Bytes(Vec<u8>),
  ChatHistory(Vec<ApronFrame>),
  ChatFrame(ApronFrame),
  ChatUser(ApronUser),
  Id(u64),
  Text(String),
  Error(String),
  None,
}
