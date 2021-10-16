mod commands;

use clap::{load_yaml, App};
use tokio::runtime::Runtime;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
  trace_init();
  info!("[slogr.io]: Starting Sentinel ....");

  let runtime = Runtime::new().unwrap();

  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml)
    .version(VERSION)
    .author(AUTHORS)
    .about(ABOUT)
    .get_matches();

  match matches.subcommand() {
    ("config", options) => debug!("config: {:?}", options),
    ("serve", options) => debug!("serve: {:?}", options),
    ("mode", options) => debug!("mode: {:?}", options),
    _ => unreachable!(),
  }

  runtime.block_on(async { twamp::io::server::Server::run().await.unwrap() });
}

fn trace_init() {
  let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
