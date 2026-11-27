use crossbeam_channel::unbounded;
use std::{
  fs,
  thread,
  time::Duration,
};
use tiny_http::Request;

use crate::collections::Config;

use crate::apron::*;
use crate::defaults::*;
use crate::protocol::*;

///////////// Server

pub(crate) struct Server {
  pub(crate) config: Config,
  pub(crate) store:  ApronStore,
}

impl Server {
  pub(crate) fn new(config: &Config) -> Result<Server,String> {
    log::info!("Initializing Chat Engine...");
    let cache_dir: String = config.get("cache_dir");
    if !cache_dir.is_empty() {
      match fs::create_dir_all(&cache_dir) {
        Ok(()) => log::info!("Created chat cache directory {} if absent.",cache_dir),
        Err(_) => return Err(format!("Failed to create chat cache directory {}.",cache_dir)),
      }
      Ok(Self {
        config: config.clone(),
        store:  ApronStore::new(&cache_dir),
      })
    } else {
      Err(format!("Cache directory not defined in config, exiting."))
    }
  }

  pub(crate) fn listen(&mut self) {
    let (incoming_tx,incoming_rx) = unbounded::<Request>();
    let (server_tx,server_rx) = unbounded::<ServerRequest>();
    let server = match tiny_http::Server::http(self.config.get("listen_addr").clone()) {
      Ok(server) => server,
      Err(err) => {
        log::error!("Failed to initialize listener: {}",err);
        return;
      },
    };
    log::info!("Listening on {}.",self.config.get("listen_addr"));
    thread::spawn(move || {
      for request in server.incoming_requests() {
        if incoming_tx.send(request).is_err() {
          log::error!("Server handler disconnected. Stopping listener.");
          break;
        }
      }
    });
    thread::spawn({
      let incoming_rx = incoming_rx.clone();
      move || {
        Self::process(&incoming_rx,&server_tx);
      }
    });
    loop {
      self.housekeeping(&server_rx);
      if server_rx.is_empty() { thread::sleep(Duration::from_secs(DEFAULT_WAIT)); }
    }
  }
}
