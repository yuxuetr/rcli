use anyhow::Result;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug, Clone)]
struct HttpServeState {
  path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
  info!("Serving {:?} on {}", &path, &port);
  let state = HttpServeState { path: path.clone() };
  let dir_service = ServeDir::new(path)
    .append_index_html_on_directories(true)
    .precompressed_gzip()
    .precompressed_br()
    .precompressed_deflate()
    .precompressed_zstd();
  let router = Router::new()
    .route("/*path", get(file_handler))
    .nest_service("/tower", dir_service)
    .with_state(Arc::new(state));
  let addr = SocketAddr::from(([0, 0, 0, 0], port));

  let listener = tokio::net::TcpListener::bind(addr).await?;
  axum::serve(listener, router).await?;
  Ok(())
}

async fn file_handler(
  State(state): State<Arc<HttpServeState>>,
  Path(path): Path<String>,
) -> (StatusCode, String) {
  let p = std::path::Path::new(&state.path).join(path);
  info!("Reading file {:?}", p);
  if !p.exists() {
    (
      StatusCode::NOT_FOUND,
      format!("File {} not found", p.display()),
    )
  } else {
    match tokio::fs::read_to_string(p).await {
      Ok(content) => {
        info!("Read {} bytes", content.len());
        (StatusCode::OK, content)
      }
      Err(e) => {
        warn!("Error reading file: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_file_handler() {
    let state = Arc::new(HttpServeState {
      path: PathBuf::from("."),
    });
    let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
    assert_eq!(status, StatusCode::OK);
    assert!(content.trim().starts_with("[package]"));
  }
}
