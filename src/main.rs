#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/foo", axum::routing::get(get_foo).post(post_foo))
        .route("/foo/bar", axum::routing::get(foo_bar))
        .route(
            "/hey/:first/:second/:third/ho/lincoln",
            axum::routing::get(hey_ho_lincoln),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn root() -> &'static str {
        "Hello Axum"
    }
    async fn get_foo() -> &'static str {
        "Getting foo"
    }
    async fn post_foo() -> &'static str {
        "Posted to foo"
    }
    async fn foo_bar() -> &'static str {
        "Getting foo bar"
    }

    async fn hey_ho_lincoln(
        axum::extract::Path((a, b, c)): axum::extract::Path<(u32, u32, u32)>,
    ) -> String {
        format!("{}, {}, {}", a, b, c)
    }
}
