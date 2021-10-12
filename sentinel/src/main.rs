mod commands;

use clap::{load_yaml, App};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

#[tokio::main]
async fn main() {
  trace_init();
  info!("Starting cli");

  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml)
    .version(VERSION)
    .author(AUTHORS)
    .about(ABOUT)
    .get_matches();

  // match matches.occurrences_of("v") {
  //   0 => println!("Verbose mode is off"),
  //   1 => println!("Verbose mode is kind of on"),
  //   2 => println!("Verbose mode is on"),
  //   _ => println!("Don't be crazy"),
  // }

  commands::serve().await;

  
}

fn trace_init() {
  let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
