use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

use ::gingersnaps::init;
use ::gingersnaps::logger;

///////////// Main

fn main() {
  logger::init();
  init::run();
}
