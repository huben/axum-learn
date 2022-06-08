use axum::{
  extract::Path,
  routing::{ get, any },
  Router,
  response::IntoResponse,
  http::{
    StatusCode,
    Uri,
  }
};

async fn captures(Path((key, id)): Path<(String, u64)>) -> String {
  format!("capture path key = {}, id = {}", key, id)
}

async fn serve_asset(Path(path): Path<String>) -> String {
  format!("wildcards path = {}", path)
}

pub fn v1() -> Router {
  let router = Router::new()
    .route("/test", get(|| async { "hello v1" }))

    ;
  router
}

pub fn v2() -> Router {
  let router = Router::new()
    .route("/test", get(|| async { "hello v2" }))

    ;
  router
}

async fn fallback(uri: Uri) -> impl IntoResponse {
  (StatusCode::NOT_FOUND, format!("File no found {}", uri))
}

pub fn new() -> Router {
  let router = Router::new()
    .route("/", get(|| async { "hello router" }))
    .route("/foo", get(|| async { "hello foo" }))

    .route("/captures/:key/:id", get(captures))

    .route("/assets/*path", get(serve_asset))

    .nest("/v1", v1())
    .nest("/v2", v2())

    .merge(v1())
    
    .fallback(any(fallback))
    ;
    
  router
}


