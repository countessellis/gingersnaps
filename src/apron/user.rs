use serde::{Deserialize,Serialize};
use std::{
  collections::HashMap,
};

use super::*;

///////////// ApronUser

#[derive(Serialize,Deserialize,Clone,Debug)]
#[serde(rename_all = "snake_case")]
pub struct ApronUser {
  pub user_id: ApronId,
  pub scheme:  String,
  pub name:    Option<String>,
  pub avatar:  Option<String>,
}

impl ApronUser {
  pub fn new(id: impl Into<ApronId>, scheme: impl Into<String>) -> Self {
    Self {
      user_id: id.into(),
      scheme:  scheme.into(),
      name:    None,
      avatar:  None,
    }
  }
}

///////////// ApronUsers

#[derive(Clone,Debug)]
pub struct ApronUsers {
  pub users: HashMap<ApronId,ApronUser>,
}

impl Default for ApronUsers {
  fn default() -> Self {
    Self {
      users:    HashMap::new(),
    }
  }
}

impl ApronUsers {
  pub fn add_user(&mut self, user: ApronUser) -> Option<ApronUser> {
    if !self.users.contains_key(&user.user_id) {
      self.users.insert(user.user_id.clone(),user.clone());
      Some(user)
    } else { None }
  }

  pub fn update_user(&mut self, user: &ApronUser) -> Option<ApronUser> {
    match self.users.get_mut(&user.user_id) {
      Some(existing) => {
        if user.name.is_some()   { existing.name = user.name.clone(); }
        if user.avatar.is_some() { existing.avatar = user.avatar.clone(); }
        Some(existing.clone())
      },
      None => None,
    }
  }

  pub fn get_user(&self, user_id: &str) -> Option<&ApronUser> {
    self.users.get(user_id)
  }

  pub fn get_user_mut(&mut self, user_id: &str) -> Option<&mut ApronUser> {
    self.users.get_mut(user_id)
  }

  pub fn remove_user(&mut self, user_id: &str) -> Option<ApronUser> {
    self.users.remove(user_id)
  }
}
