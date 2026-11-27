use serde::{Deserialize,Serialize};

use crate::defaults::*;

use super::*;

///////////// ApronRoom

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "snake_case")]
pub struct ApronRoom {
  pub room_id:     ApronId,
  pub title:       Option<String>,
  pub description: Option<String>,
}

impl Default for ApronRoom {
  fn default() -> Self {
    ApronRoom {
      room_id: "default".into(),
      title:   Some(DEFAULT_APRON_ROOM_TITLE.to_string()),
      description: Some(DEFAULT_APRON_ROOM_DESCRIPTION.to_string()),
    }
  }
}
