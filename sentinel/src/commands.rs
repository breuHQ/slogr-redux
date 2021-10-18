use clap::ArgMatches;
use tracing::debug;

pub fn serve(runtime: tokio::runtime::Runtime, options: Option<&ArgMatches>) {
  runtime.block_on(async { synthetic::io::server::Server::run().await.unwrap() });
}

pub fn client(runtime: tokio::runtime::Runtime, options: Option<&ArgMatches>) {
  runtime.block_on(async { synthetic::io::client::Client::connect().await.unwrap() });
}
