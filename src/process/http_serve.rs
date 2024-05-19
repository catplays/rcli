use std::net::SocketAddr;
use std::path::{PathBuf};
use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum::Router;
use axum::routing::get;
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServeState {
    path: PathBuf,// Path和PathBuf的区别是Path内部是一个&str, PathBuf内部是一个String,有所有权
}


pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {} path:{:?}", addr, port,path);
    let listener = tokio::net::TcpListener::bind(addr)
        .await?;

    let state = HttpServeState { path };
    let router = Router::new().route("/*path", get(file_handler))
        .with_state(Arc::new(state));


    axum::serve(listener, router).await?;
    Ok(())
}

/**
index_handler 中的参数都是实现了struct State<S>(pub S);的格式
 */
async fn file_handler(State(state): State<Arc<HttpServeState>>,
                      Path(path): Path<String>) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    format!("Reading file {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND,
         format!("File {} note found", p.display()),
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