//! Defines what can be used to create and join [`WebRoute`]s.

use std::cell::LazyCell;

use crate::{
    FixedRoute, ParameterizedRoute, fixed::segment::FixedSegment,
    parameterized::segment::ParameterizedSegment,
};

pub trait ToFixedSegments {
    /// Defines how to convert something into a [`Vec`] of [`Segment`]s.
    fn to_segments(&self) -> Vec<FixedSegment>;
}

impl ToFixedSegments for &str {
    fn to_segments(&self) -> Vec<FixedSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToFixedSegments for String {
    fn to_segments(&self) -> Vec<FixedSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(Into::into)
            .collect()
    }
}

impl ToFixedSegments for FixedRoute {
    fn to_segments(&self) -> Vec<FixedSegment> {
        self.segments()
    }
}

impl ToFixedSegments for &FixedRoute {
    fn to_segments(&self) -> Vec<FixedSegment> {
        self.segments()
    }
}

impl ToFixedSegments for LazyCell<FixedRoute> {
    fn to_segments(&self) -> Vec<FixedSegment> {
        self.segments()
    }
}

pub trait ToParameterizedSegments {
    /// Defines how to convert something into a [`Vec`] of [`ParameterizedSegment`]s.
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
