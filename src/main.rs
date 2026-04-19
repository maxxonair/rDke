pub mod math;
pub mod solver;
pub mod dke_core;
pub mod constants;
pub mod io;
pub mod environment;
pub mod util;
pub mod init;
pub mod api;

fn main() {
  api::start_server();
}
