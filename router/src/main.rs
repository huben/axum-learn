use axum::{
  Server,
};

pub mod router;

#[tokio::main]
async fn main() {
  
  Server::bind(&"0.0.0.0:3010".parse().unwrap())
    .serve(router::new().into_make_service())
    .await
    .unwrap();
}
