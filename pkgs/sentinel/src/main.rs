mod commands;

use clap::{load_yaml, App};
use tokio::runtime::Runtime;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
  prelude();

  let runtime = Runtime::new().unwrap();

  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml)
    .version(VERSION)
    .author(AUTHORS)
    .about(ABOUT)
    .get_matches();

  match matches.subcommand() {
    ("config", options) => debug!("config: {:?}", options),
    ("serve", options) => commands::serve(runtime, options),
    ("client", options) => commands::client(runtime, options),
    _ => info!("[slogr.io] Failed to start. No option selected."),
  }

  // FIXME: Proper error handling and message in case of panic.
}

/// Sets up the environment and do necessary actions before starting up the action
///
/// TODO: we need to add configuration setup. The plan to have one global config singleton object, and subsequent calls
/// to them will reference that global object.
fn prelude() {
  let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
