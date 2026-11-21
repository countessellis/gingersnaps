use crossbeam_channel::{unbounded,Receiver,Sender};
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

impl Server {
  pub(crate) fn websocket(ws: &mut WebSocket<Box<dyn ReadWrite + Send>>, path: &String, server: &Sender<ServerRequest>) {
    println!("WebSocket connected on path: {}", path);
    let (tx,rx) = unbounded::<ServerResponse>();
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
          let Ok(frame) = serde_json::from_str::<ApronFrame>(&raw_json) else {
            log::warn!("Invalid ApronFrame JSON: {}", raw_json);
            continue;
          };
          // Process the request
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
