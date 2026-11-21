use std::collections::HashMap;

use crate::util;

use crate::collections::Config;

///////////// Config

pub fn init() -> Config {
  Config::from_hashmap(&HashMap::from([
    ("ranas".to_string(),util::bin_name())
  ]))
}
