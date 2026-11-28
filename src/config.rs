use std::collections::HashMap;

use crate::util;

use crate::collections::config::*;

use crate::defaults::*;

///////////// Config

pub(crate) fn init() -> Config {
  let configurator = Configurator {
    defaults: HashMap::from([
      ("ranas".to_string(),util::bin_name()),
      ("cache_dir".to_string(),DEFAULT_CACHE_DIR.to_string()),
      ("listen_addr".to_string(),DEFAULT_LISTEN_ADDR.to_string()),
    ]),
    file: Some((DEFAULT_CONFIG_DIR.to_string(),DEFAULT_CONFIG_FILE.to_string())),
    ignore: vec![
      "--server".to_string(),
      "--help".to_string(),
      "-h".to_string(),
      "--version".to_string(),
      "-V".to_string(),
    ],
    flags: HashMap::new(),
  };
  configurator.to_config()
}
