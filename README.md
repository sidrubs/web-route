# Web Route

`web-route` provides an ergonomic way to define and manage web server routes in Rust.

Most web frameworks define routes using `&str` templates (e.g. `"/foo/{param}"`). This approach can become error-prone when you need to:

- Generate a callable version of a route (with parameters populated), for use in internal redirects, integration tests, etc.
- Join routes across nested routers — without having to worry about whether strings have leading/trailing slashes or resorting to `format!()` gymnastics.

## Features

- Cleanly define and join route segments
- Generate template routes for framework registration
- Generate populated routes with runtime parameters
- Slash handling is automatic and consistent

## Usage

For complete examples, see the [examples](https://github.com/sidrubs/web-route/tree/main/examples) and [integration tests](https://github.com/sidrubs/web-route/tree/main/tests).

```rust
use web_route::WebRoute;

// Define routes for parent and child routers
let parent_route = WebRoute::new("/parent/{param}");
let child_route = WebRoute::new("/child/route");

// Join routes — no need to worry about trailing slashes
let full_route = parent_route.join(&child_route);

// Convert to a route template (for use with frameworks like `axum`)
let template = parent_route.as_template_route();
assert_eq!(template, "/parent/{param}");

// Generate a fully-populated route with parameter values
let populated = full_route
    .as_populated_route(&RouteParams {
        param: "foobar".to_string(),
    })
    .unwrap();
assert_eq!(populated, "/parent/foobar/child/route");

#[derive(serde::Serialize)]
struct RouteParams {
    param: String,
}
```

## Potential Improvements

- Enable compile-time validation of routes and parameters for even greater safety.

## Prior Art

- [`TypedPath`](https://docs.rs/axum-extra/latest/axum_extra/routing/trait.TypedPath.html): The [`axum-extra`](https://docs.rs/axum-extra/latest/axum_extra) crate provides the `TypedPath` trait with a `#[derive(TypedPath)]` implementation. It enables type-safe, compile-time-checked population of path parameters and returns a [`Uri`](https://docs.rs/http/latest/http/uri/struct.Uri.html) suitable for use in requests. However, it does not appear to offer a clean or ergonomic way to **compose or join** multiple routes.
