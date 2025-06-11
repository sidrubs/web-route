//! Defines what can be used to create and join [`WebRoute`]s.

use std::cell::LazyCell;

use crate::{
    ParameterizedRoute, WebRoute, parameterized_route::segment::ParameterizedSegment,
    web_route::segment::WebSegment,
};

pub trait ToFixedSegments {
    /// Defines how to convert something into a [`Vec`] of [`FixedSegment`]s.
    fn to_segments(&self) -> Vec<WebSegment>;
}

impl ToFixedSegments for &str {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToFixedSegments for String {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToFixedSegments for WebRoute {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.segments()
    }
}

impl ToFixedSegments for &WebRoute {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.segments()
    }
}

impl ToFixedSegments for LazyCell<WebRoute> {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.segments()
    }
}

pub trait ToParameterizedSegments {
    /// Defines how to convert something into a [`Vec`] of
    /// [`ParameterizedSegment`]s.
    fn to_segments(&self) -> Vec<ParameterizedSegment>;
}

impl ToParameterizedSegments for &str {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToParameterizedSegments for String {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToParameterizedSegments for ParameterizedRoute {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.segments()
    }
}

impl ToParameterizedSegments for &ParameterizedRoute {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.segments()
    }
}

impl ToParameterizedSegments for LazyCell<ParameterizedRoute> {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.segments()
    }
}

// Could not do a generic implementation of `impl<T: ToFixedSegments>
// ToParameterizedSegments for T` as this clashed with `String` and `&str`
// implementations.

impl ToParameterizedSegments for WebRoute {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ToFixedSegments::to_segments(&self)
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

impl ToParameterizedSegments for &WebRoute {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ToFixedSegments::to_segments(self)
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

impl ToParameterizedSegments for LazyCell<WebRoute> {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ToFixedSegments::to_segments(self)
            .into_iter()
            .map(Into::into)
            .collect()
    }
}
