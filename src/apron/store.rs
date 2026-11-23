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
  pub rooms:      Table<Vec<ApronFrame>>,
  pub room_names: HashMap<String,u64>,
}

impl ApronStore {
  pub fn new(path: &String) -> Self {
    log::info!("Initializing Apron Store...");
    match fs::create_dir_all(&path) {
      Ok(()) => log::info!("Created cache directory {} if absent.",path),
      Err(_) => log::error!("Failed to create cache directory {}.",path),
    }
    Self {
      path:       path.clone(),
      users:      Table::new(),
      rooms:      Table::new(),
      room_names: HashMap::new(),
    }
  }

  pub fn server_info(&self) -> ApronFrame {
    ApronFrame::Server { apron: APRON_VERSION, auth: APRON_SUPPORTED_AUTH.iter().map(|auth| auth.to_string()).collect(), ping: 30 }
  }

  pub fn new_user(&mut self,scheme: &str) -> Result<ApronUser,String> {
    match scheme {
      "guest" => {
        let id = self.users.next();
        let user = ApronUser { user_id: format!("guest_{}",id) };
        self.users.insert(id,user.clone());
        Ok(user)
      },
      _ => Err(format!("Unsupported auth scheme {}",scheme)),
    }
  }

  pub fn get_room(&self, room: u64) -> Vec<ApronFrame> {
    let default_history: Vec<ApronFrame> = Vec::new();
    self.rooms.get(room).unwrap_or(&default_history).to_vec()
  }

  pub fn get_room_by_name(&self, room: &str) -> Option<u64> {
    match self.room_names.get(room) {
      Some(id) => Some(*id),
      None     => None,
    }
  }

  pub fn create_room(&mut self, room: &str) -> u64 {
    let id: u64 = self.rooms.next();
    self.room_names.insert(room.to_string(),id);
    self.rooms.insert(id,Vec::new());
    id
  }

  pub fn append_to_room(&mut self, room: u64, message: &ApronFrame) -> Option<ApronFrame> {
    let room_history = self.rooms.entry(room).or_default();
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
