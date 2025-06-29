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
