use std::{
  collections::HashMap,
  fs,
  fs::File,
  io::{BufWriter,Write},
  time::{SystemTime,UNIX_EPOCH},
};

use crate::defaults::*;
use super::*;

///////////// ApronStore

pub struct ApronStore {
  pub path:       String,
  pub users:      ApronUsers,
  pub guests:     ApronGuests,
  pub rooms:      HashMap<ApronId,(ApronRoom,Vec<ApronFrame>)>,
}

impl ApronStore {
  pub fn new(path: &String) -> Self {
    log::info!("Initializing Apron Store...");
    match fs::create_dir_all(&path) {
      Ok(()) => log::info!("Created cache directory {} if absent.",path),
      Err(_) => log::error!("Failed to create cache directory {}.",path),
    }
    let mut rooms: HashMap<ApronId,(ApronRoom,Vec<ApronFrame>)> = HashMap::new();
    let default_room: ApronRoom = Default::default();
    rooms.insert(default_room.room_id.clone(),(default_room,Vec::new()));
    Self {
      path:       path.clone(),
      users:      Default::default(),
      guests:     Default::default(),
      rooms:      rooms,
    }
  }

  pub fn server_info(&self) -> ApronFrame {
    ApronFrame::Server { apron: APRON_VERSION, auth: APRON_SUPPORTED_AUTH.iter().map(|auth| auth.to_string()).collect(), ping: 30 }
  }

  pub fn new_user(&mut self,scheme: &str) -> Result<ApronUser,String> {
    match scheme {
      "guest" => {
        match self.guests.new_guest() {
          Some(guest) => Ok(guest),
          None        => Err(format!("New guest somehow already exists")),
        }
      },
      _ => Err(format!("Unsupported auth scheme {}",scheme)),
    }
  }

  pub fn update_user(&mut self, user: &ApronUser) -> Option<ApronUser> {
    match user.scheme.as_str() {
      "guest" => self.guests.update_guest(&user),
      _       => self.users.update_user(&user),
    }
  }

  pub fn get_room(&self, room_id: &ApronId) -> (ApronRoom,Vec<ApronFrame>) {
    let mut default_room: ApronRoom = Default::default();
    default_room.room_id = room_id.clone();
    let default_history: Vec<ApronFrame> = Vec::new();
    self.rooms.get(&default_room.room_id).unwrap_or(&(default_room,default_history)).clone()
  }

  pub fn create_room(&mut self, title: Option<String>, description: Option<String>) -> ApronId {
    let id: ApronId = ApronId::from_now();
    let mut room: ApronRoom = Default::default();
    room.room_id     = id.clone();
    room.title       = title.clone();
    room.description = description.clone();
    self.rooms.insert(id.clone(),(room,Vec::new()));
    id
  }

  pub fn append_to_room(&mut self, room_id: &ApronId, message: &ApronFrame) -> Option<ApronFrame> {
    let (_,room_history) = self.rooms.entry(room_id.clone()).or_default();
    match message {
      ApronFrame::Send { body } => {
        let id = format!("{}-{}",room_id,room_history.len());
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
