use crossbeam_channel::{unbounded,Receiver,Sender};
use serde::Serialize;
use std::{
  fs,
  io::ErrorKind,
  net::TcpStream,
  str::FromStr,
  thread,
  time::Duration,
};
use tiny_http::{Header,ReadWrite,Request,Response,StatusCode};
use tungstenite::{
  protocol::Role,
  Error,
  Message,
  WebSocket,
};
use url::Url;

use super::Server;

use crate::apron::*;
use super::protocol::*;

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
      let json = serde_json::to_string(&server_frame).unwrap();
      if ws.send(Message::Text(json.into())).is_err() { return };
    }
    loop {
      match ws.read() {
        Ok(Message::Text(raw_json)) => {
          let Ok(wire) = serde_json::from_str::<WireFrame>(&raw_json) else {
            log::warn!("Invalid ApronFrame JSON: {}", raw_json);
            continue;
          };
          match wire.frame {
            ApronFrame::Auth { scheme, .. } => {
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
                  let frame: ApronFrame = ApronFrame::AuthResult {
                    you: user,
                  };
                  let json = serde_json::to_string(&frame).unwrap();
                  if ws.send(Message::Text(json.into())).is_err() { break; }
                },
                _ => {
                  if let Some(id) = wire.id {
                    if Self::send_error(ws,&id,-32601,"Unsupported auth scheme").is_err() { break; }
                  }
                },
              }
            },
            ApronFrame::Ping => {
              let json = serde_json::to_string(&ApronFrame::Pong).unwrap();
              if ws.send(Message::Text(json.into())).is_err() { break; }
            },
            _ => {
              if let Some(id) = wire.id {
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
