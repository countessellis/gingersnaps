use crate::collections::Sequence;

use super::*;

///////////// ApronGuests

#[derive(Clone,Debug)]
pub struct ApronGuests {
  pub sequence: Sequence,
  pub users:    ApronUsers,
}


impl Default for ApronGuests {
  fn default() -> Self {
    Self {
      sequence: Default::default(),
      users:    Default::default(),
    }
  }
}

impl ApronGuests {
  pub fn new_guest(&mut self) -> Option<ApronUser> {
    let id: u64 = self.sequence.next();
    let guest: ApronUser = ApronUser::new(format!("guest_{}",id),"guest");
    self.users.add_user(guest)
  }

  pub fn add_guest(&mut self, user: ApronUser) -> Option<ApronUser> {
    let parts: Vec<&str> = user.user_id.split("_").collect();
    if parts.len() > 1 && parts[0] == "guest" {
      match parts[1].parse::<u64>() {
        Ok(id) => {
          if *self.sequence < id { self.sequence.update(id); }
          self.users.add_user(user)
        },
        _ => None,
      }
    } else { None }
  }

  pub fn update_guest(&mut self, user: &ApronUser) -> Option<ApronUser> {
    self.users.update_user(user)
  }

  pub fn get_guest(&self, user_id: &str) -> Option<&ApronUser> {
    self.users.get_user(user_id)
  }

  pub fn get_guest_mut(&mut self, user_id: &str) -> Option<&mut ApronUser> {
    self.users.get_user_mut(user_id)
  }

  pub fn remove_guest(&mut self, user_id: &str) -> Option<ApronUser> {
    self.users.remove_user(user_id)
  }
}
