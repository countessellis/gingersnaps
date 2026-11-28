use crate::collections::Config;
use crate::config;

use crate::help::*;
use crate::mode::*;
use crate::server::*;
use crate::splash::*;
use crate::version::*;

///////////// Run

pub fn run() {
  log::info!("{}",version());
  let config: Config = config::init();
  log::info!("{}",config);
  let mode: Mode = Mode::mode();
  match mode {
    Mode::Server => {
      match Server::new(&config) {
        println!("{}",splash());
        println!("{}",version());
        Ok(mut server) => {
          server.listen();
        },
        Err(err) => log::error!("Failed to create server: {}",err),
      }
    },
    Mode::Help => {
      let help: Help = Help::new(&config);
      println!("{}",help);
    },
    Mode::Version => {
      println!("{}",version());
    },
  }
}
