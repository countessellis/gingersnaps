use std::{
  collections::HashMap,
  fs,
  fs::File,
  io::{BufWriter,Write},
  time::{SystemTime,UNIX_EPOCH},
};

use crate::collections::Table;
use crate::defaults::*;
use super::*;

///////////// ApronStore

pub struct ApronStore {
  pub path:       String,
  pub users:      Table<ApronUser>,
  pub rooms:      Table<(ApronRoom,Vec<ApronFrame>)>,
}

impl ApronStore {
  pub fn new(path: &String) -> Self {
    log::info!("Initializing Apron Store...");
    match fs::create_dir_all(&path) {
      Ok(()) => log::info!("Created cache directory {} if absent.",path),
      Err(_) => log::error!("Failed to create cache directory {}.",path),
    }
    let mut rooms: Table<(ApronRoom,Vec<ApronFrame>)> = Table::new();
    let mut default_room: ApronRoom = Default::default();
    default_room.room_id = "0".to_string();
    rooms.insert(0,(default_room,Vec::new()));
    Self {
      path:       path.clone(),
      users:      Table::new(),
      rooms:      rooms,
    }
  }

  pub fn server_info(&self) -> ApronFrame {
    ApronFrame::Server { apron: APRON_VERSION, auth: APRON_SUPPORTED_AUTH.iter().map(|auth| auth.to_string()).collect(), ping: 30 }
  }

  pub fn new_user(&mut self,scheme: &str) -> Result<ApronUser,String> {
    match scheme {
      "guest" => {
        let id = self.users.next();
        let user = ApronUser { user_id: format!("guest_{}",id), name: None, avatar: None };
        self.users.insert(id,user.clone());
        Ok(user)
      },
      _ => Err(format!("Unsupported auth scheme {}",scheme)),
    }
  }

  pub fn update_user(&mut self, user: &ApronUser) -> Option<ApronUser> {
    if let Some(existing) = self.users.values_mut().find(|u| u.user_id == user.user_id) {
      if user.name.is_some()   { existing.name = user.name.clone(); }
      if user.avatar.is_some() { existing.avatar = user.avatar.clone(); }
      Some(existing.clone())
    } else { None }
  }

  pub fn get_room(&self, room: u64) -> (ApronRoom,Vec<ApronFrame>) {
    let mut default_room: ApronRoom = Default::default();
    default_room.room_id = room.to_string();
    let default_history: Vec<ApronFrame> = Vec::new();
    self.rooms.get(room).unwrap_or(&(default_room,default_history)).clone()
  }

  pub fn create_room(&mut self, room: &str, title: Option<String>, description: Option<String>) -> u64 {
    let id: u64 = self.rooms.next();
    let mut room: ApronRoom = Default::default();
    room.room_id     = id.to_string();
    room.title       = title.clone();
    room.description = description.clone();
    self.rooms.insert(id,(room,Vec::new()));
    id
  }

  pub fn append_to_room(&mut self, room: u64, message: &ApronFrame) -> Option<ApronFrame> {
    let (_,room_history) = self.rooms.entry(room).or_default();
    match message {
      ApronFrame::Send { body } => {
        let id = format!("{}-{}",room,room_history.len());
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let frame: ApronFrame = ApronFrame::Message {
          id: id,
          body: body.clone(),
          author: "AuthenticatedUser".to_string(),
          timestamp: now,
        };
        room_history.push(frame.clone());
        match self.cache_msg(&frame) {
          Ok(())   => log::debug!("Message cached successfully."),
          Err(err) => log::error!("Failed to cache message: {}", err),
        };
        Some(frame)
      },
      ApronFrame::Message { id, .. } => {
        let index: usize = match room_history.binary_search_by_key(id, |frame| (&frame.id()).to_string()) {
          Ok(index) => index,
          Err(index) => index,
        };
        room_history.insert(index,message.clone());
        match self.cache_msg(&message) {
          Ok(())   => log::debug!("Message cached successfully."),
          Err(err) => log::error!("Failed to cache message: {}", err),
        };
        Some(message.clone())
      },
      _ => None,
    }
  }

  pub fn cache_msg(&self, frame: &ApronFrame) -> Result<(),String> {
    match frame {
      ApronFrame::Message { id, .. } => {
        match File::create(format!("{}/{}.msg",self.path,id)) {
          Ok(file) => {
            let mut writer = BufWriter::new(file);
            match serde_json::to_vec(frame) {
              Ok(bytes) => {
                match writer.write_all(&bytes) {
                  Ok(()) => match writer.flush() {
                    Ok(()) => Ok(()),
                    Err(err) => Err(format!("Failed to flush message to disk: {}", err)),
                  },
                  Err(err) => Err(format!("Failed to write message to disk: {}",err)),
                }
              },
              Err(err) => Err(format!("Failed to serialize frame: {}",err)),
            }
          },
          Err(err) => Err(format!("Failed to open file: {}",err)),
        }
      }
      _ => Err(format!("Only messages can be cached.")),
    }
  }
}
