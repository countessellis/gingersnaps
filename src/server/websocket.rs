use crossbeam_channel::{unbounded,Sender};
use serde::Serialize;
use tiny_http::ReadWrite;
use tungstenite::{
  Error,
  Message,
  WebSocket,
};

use super::Server;

use crate::apron::*;
use crate::protocol::*;

///////////// Server

#[derive(Serialize)]
struct ErrorReply<'a> {
  id: &'a str,
  error: ErrorBody<'a>,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
  code: i64,
  message: &'a str,
}

impl Server {
  pub(crate) fn send_error(ws: &mut WebSocket<Box<dyn ReadWrite + Send>>, req_id: &str, code: i64, message: &str) -> Result<(),tungstenite::Error> {
    let reply = ErrorReply {
      id: req_id,
      error: ErrorBody { code, message },
    };
    let json = serde_json::to_string(&reply).unwrap();
    log::debug!("Sending Error: {}",json);
    ws.send(Message::Text(json.into()))
  }

  pub(crate) fn send_notification(ws: &mut WebSocket<Box<dyn ReadWrite + Send>>, frame: &ApronFrame) -> Result<(),tungstenite::Error> {
    let json = serde_json::to_string(frame).unwrap();
    log::debug!("Sending Notification: {}",json);
    ws.send(Message::Text(json.into()))
  }

  pub(crate) fn send_result<T: serde::Serialize>(ws: &mut WebSocket<Box<dyn ReadWrite + Send>>, req_id: &str, frame: &T,) -> Result<(), tungstenite::Error> {
    let reply = ReplyFrame { id: req_id, result: frame };
    let json = serde_json::to_string(&reply).unwrap();
    log::debug!("Sending Result: {}", json);
    ws.send(Message::Text(json.into()))
  }

  pub(crate) fn websocket(ws: &mut WebSocket<Box<dyn ReadWrite + Send>>, path: &String, server: &Sender<ServerRequest>) {
    println!("WebSocket connected on path: {}", path);
    let (tx,rx) = unbounded::<ServerResponse>();
    let mut you: Option<ApronUser> = None;
    let server_info = ServerRequest {
      command: "server_info".to_string(),
      args: ServerArgs::None,
      sender: tx,
    };
    log::debug!("Requesting server info from main thread.");
    if server.send(server_info).is_err() { return; }
    if let Ok(ServerResponse::ChatFrame(server_frame)) = rx.recv() {
      log::debug!("Received server frame from main thread: {:?}",server_frame);
      if Self::send_notification(ws,&server_frame).is_err() { return; }
    }
    loop {
      match ws.read() {
        Ok(Message::Text(json)) => {
          log::debug!("Received: {}",json);
          let Ok(wire) = serde_json::from_str::<WireFrame>(&json) else {
            log::warn!("Invalid ApronFrame JSON: {}",json);
            continue;
          };
          let id = wire.id.clone();
          match wire.into_frame() {
            ApronFrame::Auth { scheme, .. } => {
              if let Some(id) = id {
                let (tx,rx) = unbounded::<ServerResponse>();
                let new_user = ServerRequest {
                  command: "new_user".to_string(),
                  args: ServerArgs::Text(scheme.clone()),
                  sender: tx,
                };
                if server.send(new_user).is_err() { return; }
                match rx.recv() {
                  Ok(ServerResponse::ChatUser(user)) => {
                    you = Some(user.clone());
                    let result = serde_json::json!({ "you": user });
                    if Self::send_result(ws,&id,&result).is_err() { break; }
                  },
                  _ => {
                    if Self::send_error(ws,&id,-32601,"Unsupported auth scheme").is_err() { break; }
                  },
                }
              }
            },
            ApronFrame::Me { name, avatar } => {
              if let Some(id) = id {
                if let Some(mut user) = you.clone() {
                  if let Some(name)   = name { user.name = Some(name.clone()) }
                  if let Some(avatar) = avatar { user.avatar = Some(avatar.clone()) }
                  let (tx,rx) = unbounded::<ServerResponse>();
                  let update_user = ServerRequest {
                    command: "update_user".to_string(),
                    args: ServerArgs::ChatUser(user),
                    sender: tx,
                  };
                  if server.send(update_user).is_err() { return; }
                  match rx.recv() {
                    Ok(ServerResponse::ChatUser(user)) => {
                      you = Some(user.clone());
                      let result = serde_json::json!({ "you": user });
                      if Self::send_result(ws,&id,&result).is_err() { break; }
                    },
                    _ => {
                      if Self::send_error(ws,&id,-32603,"Failed to update you").is_err() { break; }
                    },
                  }
                } else {
                  if Self::send_error(ws,&id,-32001,"Please authenticate before sending").is_err() { break; }
                }
              }

            },
            ApronFrame::Ping {} => {
              if Self::send_notification(ws,&ApronFrame::Pong).is_err() { break; }
            },
            _ => {
              if let Some(id) = id {
                if you.is_none() {
                  if Self::send_error(ws,&id,-32001,"Please authenticate before sending").is_err() { break; }
                } else {
                  if Self::send_error(ws,&id,-32601,"Unsupported method").is_err() { break; }
                }
              }
            },
          }
        },
        Ok(Message::Close(_)) | Err(Error::ConnectionClosed) => {
          println!("WebSocket disconnected on path: {}", path);
          break;
        },
        Err(error) => {
          log::warn!("WebSocket read error: {}", error);
          break;
        }
        _ => {},
      }
    }
  }
}
