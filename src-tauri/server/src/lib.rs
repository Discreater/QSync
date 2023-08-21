use std::{net::SocketAddr, sync::Arc, time::Duration};

use abi::musync_service_server::MusyncServiceServer;
use axum::{routing::get, Extension, Router};

use axum_server::Handle;
use error::Result;
use http::{
  header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  HeaderName, Request,
};
use hyper::Body;
use tokio::task::JoinHandle;
use tonic_web::GrpcWebLayer;
use tower::{make::Shared, steer::Steer, BoxError, ServiceExt};
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};
use tracing::info;

use crate::grpc::GrpcServer;

mod error;
mod grpc;
mod musync;
mod user;
mod ws;

pub struct Server {
  pub join_handle: JoinHandle<std::result::Result<(), std::io::Error>>,
  pub handle: Handle,
}

impl Server {
  /// spawn a new task, return the join handle of the server and the server handle
  pub async fn serve(addr: &SocketAddr, db_url: &str) -> Result<Self> {
    let manager = dbm::DbManager::from_url(db_url).await?;

    let cors = CorsLayer::new()
      .allow_headers([
        AUTHORIZATION,
        ACCEPT,
        CONTENT_TYPE,
        HeaderName::from_static("x-grpc-web"),
      ])
      .allow_methods(AllowMethods::any())
      .allow_origin(AllowOrigin::any())
      .max_age(Duration::from_secs(60) * 10);

    let ws_state = Arc::new(ws::WsState::default());

    let grpc_service = tonic::transport::Server::builder()
      .accept_http1(true)
      .layer(cors.clone())
      .layer(GrpcWebLayer::new())
      .add_service(MusyncServiceServer::with_interceptor(
        GrpcServer::new(manager.clone(), Arc::clone(&ws_state)),
        grpc::check_auth,
      ))
      .into_service()
      .map_response(|r| r.map(axum::body::boxed))
      .boxed_clone();

    let http_app = Router::new()
      .nest(
        "/assets",
        Router::new()
          .route("/track/:id", get(musync::get_track))
          .route("/track/:id/cover", get(musync::get_cover))
          .with_state(manager.clone()),
      )
      .route("/ws", get(ws::handler).layer(Extension(ws_state)))
      .with_state(manager.clone())
      .layer(cors);

    let http_service = http_app.map_err(BoxError::from).boxed_clone();

    let hybrid_service = Steer::new(
      vec![http_service, grpc_service],
      |req: &Request<Body>, _svcs: &[_]| {
        let content_type = req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes());
        if content_type != Some(b"application/grpc")
          && content_type != Some(b"application/grpc-web")
          && content_type != Some(b"application/grpc-web+proto")
        {
          0
        } else {
          1
        }
      },
    );
    let handle = Handle::new();
    let server = axum_server::bind(*addr)
      .handle(handle.clone())
      .serve(Shared::new(hybrid_service));
    let join_handle = tokio::spawn(server);
    // wait until server is listening.
    let addr = handle.listening().await;
    info!("Server listening on {:?}", addr);

    Ok(Self {
      join_handle,
      handle,
    })
  }
}

#[cfg(test)]
mod tests {
  use hyper::{body::HttpBody, Body, Method, Request};

  use super::*;

  fn init_tracing() {
    tracing_subscriber::fmt()
      .with_env_filter("server=trace,hyper=warn")
      .init();
  }

  #[tokio::test]
  async fn main() {
    init_tracing();
    let server = {
      let addr = SocketAddr::from(([127, 0, 0, 1], 8396));
      let memory_sqlite_url = "sqlite::memory:";
      Server::serve(&addr, memory_sqlite_url).await.unwrap()
    };
    let client = tokio::spawn(async move {
      let client = hyper::Client::new();
      let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:8396/api/musync/folders")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"path": "D:\\media\\music"}"#))
        .unwrap();
      let res = client.request(req).await.unwrap();
      assert_eq!(res.status(), 201);
      let req = Request::builder()
        .method(Method::POST)
        .uri("http://127.0.0.1:8396/api/musync/tracks")
        .header("content-type", "application/json")
        .body(Body::from("{}"))
        .unwrap();
      let res = client.request(req).await.unwrap();
      assert_eq!(res.status(), 200);
      let mut response_body = String::new();
      let mut body = res.into_body();
      while let Some(chunk) = body.data().await {
        let chunk = chunk.unwrap();
        response_body.push_str(std::str::from_utf8(&chunk).unwrap());
      }
      let tracks: Vec<abi::musync::Track> = serde_json::from_str(&response_body).unwrap();
      assert!(!tracks.is_empty())
    });
    tokio::select! {
      _ = server.join_handle => {},
      _ = client => {
        server.handle.graceful_shutdown(None)
      },
    }
  }
}
