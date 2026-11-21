use crate::collections::Config;
use crate::config;

use crate::help::*;
use crate::mode::*;
use crate::splash::*;
use crate::version::*;

///////////// Run

pub fn run() {
  println!("{}",splash());
  println!("{}",version());
  log::info!("{}",version());
  let config: Config = config::init();
  log::info!("{}",config);
  let mode: Mode = Mode::mode();
  match mode {
    Mode::Server => {
    },
    Mode::Help => {
      let help: Help = Help::new(&config);
      println!("{}",help);
    },
  }
}
