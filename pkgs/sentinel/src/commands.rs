use clap::ArgMatches;
use tracing::debug;

pub fn serve(runtime: tokio::runtime::Runtime, _options: Option<&ArgMatches>) {
  runtime.block_on(async { synthetic::io::server::Server::run().await.unwrap() });
}

pub fn client(runtime: tokio::runtime::Runtime, _options: Option<&ArgMatches>) {
  runtime.block_on(async {
    let result = synthetic::io::client::Client::connect().await;
    match result {
      Ok(()) => debug!("Client [Shutdown]"),
      Err(e) => debug!("Client [Error]: {:?}", e),
    }
  });
}
