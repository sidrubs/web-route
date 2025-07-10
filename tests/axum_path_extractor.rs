//! Ensures that a [`WebRoute`] can be extracted by an `axum` [`Path`] path
//! extractor.

use std::sync::LazyLock;

use axum::{Json, Router, extract::Path, routing::get};
use web_route::{ParameterizedRoute, WebRoute};

// Would be cool if we could make this able to be evaluated at compile time so
// that this can be a const without `LazyCell`.
static ROUTE_WITH_PATH: LazyLock<ParameterizedRoute> =
    LazyLock::new(|| ParameterizedRoute::new("/foo/{*path}"));

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct RouteParams {
    path: WebRoute,
}

async fn route_handler(Path(params): Path<RouteParams>) -> Json<RouteParams> {
    Json(params)
}

fn build_router() -> Router {
    Router::new().route(&ROUTE_WITH_PATH, get(route_handler))
}

#[tokio::test]
async fn should_be_able_to_extract_a_web_route_with_axum_path_extractor() {
    // Arrange
    let path_params = RouteParams {
        path: WebRoute::new("another/route"),
    };

    let test_server = axum_test::TestServer::new(build_router()).unwrap();

    // Act
    let response = test_server
        .get(
            // Using `WebRoute` to build a route with the parameters populated.
            &ROUTE_WITH_PATH.to_web_route(&path_params).unwrap(),
        )
        .await;

    // Assert
    let parsed_body = response.json::<RouteParams>();
    assert_eq!(parsed_body, path_params);
}
