use std::net::SocketAddr;
use std::path::{PathBuf};
use std::sync::Arc;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use axum::body::Body;
use axum::response::Response;
use axum::Router;
use axum::routing::{get, get_service};
use tower_http::services::ServeDir;
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

    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        // /*path 只能匹配/a、/b/a/ 不能匹配到"/"。 所以对于/要不使用路由注册，或者匹配不上fallback。
        .route("/v1/*path", get(file_handler))
        .fallback(index_page)
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));


    axum::serve(listener, router).await?;
    Ok(())
}


async fn index_page(State(state): State<Arc<HttpServeState>>) -> Response{
    let root = std::path::Path::new(&state.path);
    let mut html_rsp:Vec<String> = Vec::new();
    html_rsp.push(String::from("<html><body> <h1>目录</h1>"));
    if root.is_dir() {
        let paths = root.read_dir().unwrap();
        for p in paths {
            let entry = p.unwrap();
            if entry.file_type().unwrap().is_file() {
                println!("file:{}", entry.file_name().to_str().unwrap());
                html_rsp.push(String::from("<p>"));
                html_rsp.push(entry.file_name().into_string().unwrap());
                html_rsp.push(String::from("</p>"));
            }
        }
    }
    html_rsp.push(String::from("</body> </html>"));
    let html = html_rsp.join("\n");
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(html))
        .unwrap()
}

/**
file_handler 中的参数都是实现了struct State<S>(pub S);的格式
 */
async fn file_handler(State(state): State<Arc<HttpServeState>>,
                      Path(path): Path<String>) -> (StatusCode, String) {


    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    println!("Reading file {:?}", p);
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

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::sync::Arc;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use crate::process::http_serve::{file_handler, HttpServeState};

    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from(".")
        });

        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"))
    }
}