use std::net::SocketAddr;

fn init_tracing() {
  tracing_subscriber::fmt()
    .with_env_filter("server=trace,hyper=warn,dbm=trace")
    .init();
}

#[tokio::main]
async fn main() {
  init_tracing();
  // load envrionment
  dotenvy::dotenv().expect(".env file not found");
  let addr = SocketAddr::from(([127, 0, 0, 1], 8396));
  let sqlite_file = "./db.sqlite";
  if !std::path::Path::new(sqlite_file).exists() {
    std::fs::File::create(sqlite_file).unwrap();
  }
  let sqlite_url: String = format!("sqlite://{}", sqlite_file);
  let server = server::Server::serve(&addr, &sqlite_url).await.unwrap();

  tokio::join!(server.join_handle).0.unwrap().unwrap();
}
