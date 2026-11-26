use serde::{Deserialize,Serialize};

use crate::defaults::*;

///////////// ApronRoom

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "snake_case")]
pub struct ApronRoom {
  pub room_id:     String,
  pub title:       Option<String>,
  pub description: Option<String>,
}

impl Default for ApronRoom {
  fn default() -> Self {
    ApronRoom {
      room_id: "0".to_string(),
      title:   Some(DEFAULT_APRON_ROOM_TITLE.to_string()),
      description: Some(DEFAULT_APRON_ROOM_DESCRIPTION.to_string()),
    }
  }
}
