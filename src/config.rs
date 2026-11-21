use std::collections::HashMap;

use crate::util;

use crate::collections::Config;

use crate::defaults::*;

///////////// Config

pub(crate) fn init() -> Config {
  Config::from_hashmap(&HashMap::from([
    ("ranas".to_string(),util::bin_name()),
    ("cache_dir".to_string(),DEFAULT_CACHE_DIR.to_string()),
    ("listen_addr".to_string(),DEFAULT_LISTEN_ADDR.to_string()),
  ]))
}
