#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello Axum!" }));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
