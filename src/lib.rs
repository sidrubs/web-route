#![doc = include_str!("../README.md")]

pub mod error;
pub mod fixed;
pub mod parameterized;
mod to_segments;
mod utils;

pub use fixed::route::FixedRoute;
pub use parameterized::route::ParameterizedRoute;
