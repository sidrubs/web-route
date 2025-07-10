//! An example application that uses the same [`WebRoute`]s to define both an
//! `axum` router and build out a populated redirect route.

use std::sync::LazyLock;

use axum::{
    Router,
    extract::Path,
    response::{Html, Redirect},
    routing::get,
};
use web_route::ParameterizedRoute;

// Would be cool if we could make this able to be evaluated at compile time so
// that this can be a static without `LazyLock`.
static FOO_ROUTE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/foo/{foo_id}"));
static BAR_ROUTE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/bar/{bar_id}"));
static BAZ_ROUTE: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/baz/{bar_id}"));

fn build_router() -> Router {
    // Using the `WebRoute` to define axum server routes.
    let nested_router = Router::new()
        .route(&BAR_ROUTE, get(bar_handler))
        .route(&BAZ_ROUTE, get(baz_handler));

    Router::new().nest(&FOO_ROUTE, nested_router)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RouteParams {
    foo_id: String,
    bar_id: String,
}

async fn bar_handler(Path(params): Path<RouteParams>) -> Redirect {
    Redirect::to(
        // Using the `WebRoute` to populate the redirect route.
        &FOO_ROUTE
            .join(BAZ_ROUTE.clone())
            .to_web_route(&params)
            .unwrap(),
    )
}

async fn baz_handler(Path(params): Path<RouteParams>) -> Html<String> {
    Html(format!("<h1>{params:?}</h1>"))
}

#[tokio::main]
async fn main() {
    let app = build_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    println!(
        "Navigate to /foo/{{foo_id}}/bar/{{bar_id}} and be redirected to /foo/{{foo_id}}/baz/{{bar_id}} using a `WebRoute`"
    );
    axum::serve(listener, app).await.unwrap();
}
