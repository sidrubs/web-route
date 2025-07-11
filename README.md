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

One can create and join [`WebRoute`]s without having to worry about leading and trailing slash pedantics. The resulting routes will always be normalized to have single forward slash separators no matter the operating system.

```rust
use web_route::WebRoute;

let foo = WebRoute::new("no/leading/slash/");
let bar = WebRoute::new("/leading/and//trailing/slash/");

// Can join `WebRoute`s.
let joined_route = foo.join(bar);
assert_eq!(&joined_route.to_string(), "/no/leading/slash/leading/and/trailing/slash");

// Can join `String`s and `&str`.
let joined_route = foo.join("/a/str/route");
assert_eq!(&joined_route.to_string(), "/no/leading/slash/a/str/route");
```

[`WebRoute`]s can be serialized and deserialized.

```rust
use serde::{Serialize, Deserialize};
use web_route::WebRoute;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FooStruct {
    route: WebRoute
}

let foo_struct = FooStruct {
    route: WebRoute::new("/foo/bar/baz"),
};

let serialized = serde_json::to_string(&foo_struct).unwrap();
let deserialized = serde_json::from_str::<FooStruct>(&serialized).unwrap();

assert_eq!(deserialized, foo_struct);
```

One would use a [`ParameterizedRoute`][] when defining routes used by a webserver (e.g. [`axum`](https://github.com/tokio-rs/axum))

```rust
use web_route::ParameterizedRoute;

let foo = ParameterizedRoute::new("/foo/{foo_id}");
let bar = ParameterizedRoute::new("/bar/{bar_id}");

assert_eq!(&foo.join(bar).to_string(), "/foo/{foo_id}/bar/{bar_id}");
```

A [`ParameterizedRoute`][] can be populated with values to produce a [`WebRoute`] which can then be used to make a request to the server route it defines.

```rust
use serde::{Serialize, Deserialize};
use web_route::ParameterizedRoute;

let foo = ParameterizedRoute::new("/foo/{foo_id}");
let bar = ParameterizedRoute::new("/bar/{bar_id}");

#[derive(Serialize, Deserialize)]
struct Params {
    foo_id: String,
    bar_id: String,
}

let params = Params {
    foo_id: "value_foo".to_owned(),
    bar_id: "value_bar".to_owned(),
};

let web_route = foo.join(bar).to_web_route(&params).unwrap();

assert_eq!(&web_route.to_string(), "/foo/value_foo/bar/value_bar");
```

For more complete examples, see the [examples](https://github.com/sidrubs/web-route/tree/main/examples) and [integration tests](https://github.com/sidrubs/web-route/tree/main/tests).


## Potential Improvements

- Enable compile-time validation of routes and parameters for even greater safety.

## Prior Art

- [`TypedPath`](https://docs.rs/axum-extra/latest/axum_extra/routing/trait.TypedPath.html): The [`axum-extra`](https://docs.rs/axum-extra/latest/axum_extra) crate provides the `TypedPath` trait with a `#[derive(TypedPath)]` implementation. It enables type-safe, compile-time-checked population of path parameters and returns a [`Uri`](https://docs.rs/http/latest/http/uri/struct.Uri.html) suitable for use in requests. However, it does not appear to offer a clean or ergonomic way to **compose or join** multiple routes.

[`WebRoute`]: ./src/web_route/route.rs
[`ParameterizedRoute`]: ./src/parameterized_route/route.rs
