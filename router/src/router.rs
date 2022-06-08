use axum::{
  routing::get,
  Router,
};

pub fn new() -> Router {
  let router = Router::new()
    .route("/", get(|| async { "hello router" }));

    
  router
}


