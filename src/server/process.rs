use crossbeam_channel::{Receiver,Sender};
use std::{
  str::FromStr,
  thread,
};
use tiny_http::{Header,Request,Response,StatusCode};
use tungstenite::{
  protocol::Role,
  WebSocket,
};
use url::Url;

use super::Server;

use crate::protocol::*;
use crate::defaults::*;

///////////// Server

impl Server {
  pub(crate) fn process(incoming: &Receiver<Request>, server: &Sender<ServerRequest>) {
    for request in incoming.iter() {
      if let Some(remote) = request.remote_addr() {
        log::debug!("Received connection from {}:{}",remote.ip(),remote.port());
        let base_url = Url::parse("http://localhost").unwrap();
        if let Ok(parsed_url) = base_url.join(request.url()) {
          let path = parsed_url.path().to_string();
          log::info!("Request received for on {}",path);
          thread::spawn({
            let server = server.clone();
            move || {
              let is_ws_upgrade = request.method() == &tiny_http::Method::Get && request.headers().iter().any(|header| header.field.equiv("Upgrade") && header.value.as_str().eq_ignore_ascii_case("websocket"));
              if is_ws_upgrade {
                let is_valid_version = request.headers().iter().any(|header| { header.field.equiv("Sec-WebSocket-Version") && header.value.as_str().trim() == "13" });
                if !is_valid_version {
                  let response = Response::empty(StatusCode(426)).with_header(Header::from_str("Sec-WebSocket-Version: 13").unwrap());
                  let _ = request.respond(response);
                  return;
                }
                if let Some(key) = request.headers().iter().find(|header| header.field.equiv("Sec-WebSocket-Key")).map(|header| header.value.as_str().to_string()) {
                  let accept_key = tungstenite::handshake::derive_accept_key(key.as_bytes());
                  let response = Response::empty(StatusCode(101))
                    .with_header(Header::from_str("Upgrade: websocket").unwrap())
                    .with_header(Header::from_str("Connection: Upgrade").unwrap())
                    .with_header(Header::from_str(&format!("Sec-WebSocket-Accept: {}", accept_key)).unwrap())
                    .with_header(Header::from_str(&format!("Server: {}/{}",APP_NAME,VERSION_ID)).unwrap());
                  let stream = request.upgrade("websocket", response);
                  let mut ws = WebSocket::from_raw_socket(stream, Role::Server, None);
                  Self::websocket(&mut ws,&path,&server);
                  return;
                }
              }
            }
          });
        }
      }
    }
  }
}
