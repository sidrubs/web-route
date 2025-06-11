//! An example application that uses the same [`WebRoute`]s to define both an
//! `axum` router and build out a populated redirect route.

use std::cell::LazyCell;

use axum::{
    Router,
    extract::Path,
    response::{Html, Redirect},
    routing::get,
};
use web_route::ParameterizedRoute;

// Would be cool if we could make this able to be evaluated at compile time so
// that this can be a const without `LazyCell`.
const FOO_ROUTE: LazyCell<ParameterizedRoute> =
    LazyCell::new(|| ParameterizedRoute::new("/foo/{foo_id}"));
const BAR_ROUTE: LazyCell<ParameterizedRoute> =
    LazyCell::new(|| ParameterizedRoute::new("/bar/{bar_id}"));
const BAZ_ROUTE: LazyCell<ParameterizedRoute> =
    LazyCell::new(|| ParameterizedRoute::new("/baz/{bar_id}"));

fn build_router() -> Router {
    // Using the `WebRoute` to define axum server routes.
    let nested_router = Router::new()
        .route(&BAR_ROUTE.to_string(), get(bar_handler))
        .route(&BAZ_ROUTE.to_string(), get(baz_handler));
    let root_router = Router::new().nest(&FOO_ROUTE.to_string(), nested_router);

    root_router
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
            .join(BAZ_ROUTE)
            .to_web_route(&params)
            .unwrap()
            .to_string(),
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
