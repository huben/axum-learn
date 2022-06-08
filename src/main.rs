use axum::{
  routing::get,
  Router,
  Server,
};

#[tokio::main]
async fn main() {
  let router = Router::new()
    .route("/", get(|| async { "hello world" }));

  Server::bind(&"0.0.0.0:3010".parse().unwrap())
    .serve(router.into_make_service())
    .await
    .unwrap();
}
