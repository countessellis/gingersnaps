use serde::{Deserialize,Serialize};
use std::{
  fmt,
};

///////////// ApronUser

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "snake_case")]
pub struct ApronUser {
  pub user_id: String,
}
