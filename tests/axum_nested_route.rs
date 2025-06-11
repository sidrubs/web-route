//! Defines a simple nested `axum` router using [`WebRoute`]s to define the
//! routes. An integration test uses these same [`WebRoute`]s to make a request
//! to the endpoint.

use std::cell::LazyCell;

use axum::{Json, Router, extract::Path, routing::get};
use fake::{Fake, Faker};
use web_route::FixedRoute;

// Would be cool if we could make this able to be evaluated at compile time so
// that this can be a const without `LazyCell`.
const FOO_ROUTE: LazyCell<FixedRoute> = LazyCell::new(|| FixedRoute::new("/foo/{foo_id}"));
const BAR_ROUTE: LazyCell<FixedRoute> = LazyCell::new(|| FixedRoute::new("/bar/{bar_id}"));

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy)]
struct RouteParams {
    foo_id: String,
    bar_id: String,
}

async fn route_handler(Path(params): Path<RouteParams>) -> Json<RouteParams> {
    Json(params)
}

fn build_router() -> Router {
    // Using the `WebRoute` to define axum server routes.
    let nested_router = Router::new().route(&BAR_ROUTE.as_template_route(), get(route_handler));
    let root_router = Router::new().nest(&FOO_ROUTE.as_template_route(), nested_router);

    root_router
}

#[tokio::test]
async fn should_be_able_to_generate_populated_route() {
    // Arrange
    let path_params = Faker.fake::<RouteParams>();

    let test_server = axum_test::TestServer::new(build_router()).unwrap();

    // Act
    let response = test_server
        .get(
            // Using `WebRoute` to build a route with the parameters populated.
            &FOO_ROUTE
                .join(BAR_ROUTE)
                .as_evaluated_route(&path_params)
                .unwrap(),
        )
        .await;

    // Assert
    let parsed_body = response.json::<RouteParams>();
    assert_eq!(parsed_body, path_params);
}
