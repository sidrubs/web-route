//! [`WebRoute`]: WebRoute
//! [`ParameterizedRoute`]: ParameterizedRoute
#![doc = include_str!("../README.md")]

pub mod error;
pub mod parameterized_route;
mod to_segments;
mod utils;
pub mod web_route;

pub use parameterized_route::route::ParameterizedRoute;
pub use web_route::route::WebRoute;

/// Generate something like this with a macro.
///
/// Joins would be by another macro and make a completely new struct.
///
/// Need some method of adding to a
pub struct ExampleRoute;

impl ExampleRoute {
    //
    pub fn to_populated_route(params: &RouteParameters) -> String {
        let RouteParameters { param_1, param_2 } = params;

        format!("/this/{param_1}/route/{param_2}")
    }

    /// Would be used to format the route in an axum specific manner. Could have functions for other web frameworks.
    ///
    /// Not sure what the most idiomatic way of doing this is.
    pub fn to_axum() -> &'static str {
        "/this/{param_1}/route/{param_2}"
    }
}

/// This can be used to make a populated version of the route as well as an extractor from URL path.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteParameters {
    pub param_1: String,
    pub param_2: String,
}
