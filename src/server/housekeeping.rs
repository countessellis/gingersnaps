use crossbeam_channel::{unbounded,Receiver,Sender};
use std::{
  fs,
  thread,
  time::Duration,
};
use tiny_http::Request;
use url::Url;

use super::Server;

use super::protocol::*;

///////////// Server

impl Server {
  pub(crate) fn housekeeping(&mut self, server: &Receiver<ServerRequest>) {
    log::debug!("-----------HOUSEKEEPING-----------");
    log::debug!("---HOUSEKEEPING-SERVER-REQUESTS---");
    for request in server.try_iter() {
      match request.command.as_str() {
        "server_info" => {
          let response: ServerResponse = ServerResponse::ChatFrame(self.store.server_info());
          if request.sender.send(response).is_err() {
            log::error!("Server handler disconnected.");
          }
        },
        "new_user" => {
          let response: ServerResponse = match request.args {
            ServerArgs::Text(scheme) => match self.store.new_user(&scheme) {
              Ok(user) => ServerResponse::ChatUser(user),
              Err(err) => ServerResponse::Error(err),
            }
            _ => ServerResponse::Error("Invalid arguments.".to_string()),
          };
          if request.sender.send(response).is_err() {
            log::error!("Server handler disconnected.");
          }
        },
        "update_user" => {
          let response: ServerResponse = match request.args {
            ServerArgs::ChatUser(user) => match self.store.update_user(&user) {
              Some(user) => ServerResponse::ChatUser(user),
              None       => ServerResponse::None,
            }
            _ => ServerResponse::Error("Invalid arguments.".to_string()),
          };
          if request.sender.send(response).is_err() {
            log::error!("Server handler disconnected.");
          }
        },
        "create_room" => {
          let response: ServerResponse = match request.args {
            ServerArgs::Text(room) => {
              ServerResponse::Id(self.store.create_room(&room))
            },
            _ => ServerResponse::Error("Invalid arguments.".to_string()),
          };
          if request.sender.send(response).is_err() {
            log::error!("Server handler disconnected.");
          }
        },
        "get_room" => {
          let response: ServerResponse = match request.args {
            ServerArgs::Id(room) => {
              ServerResponse::ChatHistory(self.store.get_room(room))
            },
            _ => ServerResponse::Error("Invalid arguments.".to_string()),
          };
          if request.sender.send(response).is_err() {
            log::error!("Server handler disconnected.");
          }
        },
        "get_room_by_name" => {
          let response: ServerResponse = match request.args {
            ServerArgs::Text(room) => {
              match self.store.get_room_by_name(&room) {
                Some(id) => ServerResponse::Id(id),
                None     => ServerResponse::None,
              }
            },
            _ => ServerResponse::Error("Invalid arguments.".to_string()),
          };
          if request.sender.send(response).is_err() {
            log::error!("Server handler disconnected.");
          }
        },
        _ => log::error!("Received invalid server command: {}",request.command),
      }
    }
    log::debug!("---------HOUSEKEEPING-END---------");
  }
}
