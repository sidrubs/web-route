//! Defines what can be used to create and join [`WebRoute`]s.

use std::{cell::LazyCell, sync::LazyLock};

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
            .map(TryInto::try_into)
            .filter_map(|res| res.ok())
            .collect()
    }
}

impl ToFixedSegments for String {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(TryInto::try_into)
            .filter_map(|res| res.ok())
            .collect()
    }
}

impl ToFixedSegments for WebRoute {
    fn to_segments(&self) -> Vec<WebSegment> {
        WebRoute::to_segments(self)
    }
}

impl ToFixedSegments for &WebRoute {
    fn to_segments(&self) -> Vec<WebSegment> {
        WebRoute::to_segments(self)
    }
}

impl ToFixedSegments for LazyCell<WebRoute> {
    fn to_segments(&self) -> Vec<WebSegment> {
        WebRoute::to_segments(self)
    }
}

impl ToFixedSegments for LazyLock<WebRoute> {
    fn to_segments(&self) -> Vec<WebSegment> {
        WebRoute::to_segments(self)
    }
}

#[cfg(feature = "fake")]
impl ToFixedSegments for Vec<WebSegment> {
    fn to_segments(&self) -> Vec<WebSegment> {
        self.clone()
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
            .map(TryInto::try_into)
            .filter_map(|res| res.ok())
            .collect()
    }
}

impl ToParameterizedSegments for String {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.trim_start_matches("/")
            .trim_end_matches("/")
            .split("/")
            .map(TryInto::try_into)
            .filter_map(|res| res.ok())
            .collect()
    }
}

impl ToParameterizedSegments for ParameterizedRoute {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ParameterizedRoute::to_segments(self)
    }
}

impl ToParameterizedSegments for &ParameterizedRoute {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ParameterizedRoute::to_segments(self)
    }
}

impl ToParameterizedSegments for LazyCell<ParameterizedRoute> {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ParameterizedRoute::to_segments(self)
    }
}

impl ToParameterizedSegments for LazyLock<ParameterizedRoute> {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ParameterizedRoute::to_segments(self)
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

impl ToParameterizedSegments for LazyLock<WebRoute> {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        ToFixedSegments::to_segments(self)
            .into_iter()
            .map(Into::into)
            .collect()
    }
}

#[cfg(feature = "fake")]
impl ToParameterizedSegments for Vec<ParameterizedSegment> {
    fn to_segments(&self) -> Vec<ParameterizedSegment> {
        self.clone()
    }
}

#[cfg(test)]
mod to_fixed_segment_tests {
    use crate::to_segments::ToFixedSegments;

    #[test]
    fn str_should_normalize_double_slashes() {
        // Arrange
        let non_normalized = "/foo//bar";

        // Act
        let segments = ToFixedSegments::to_segments(&non_normalized);

        // Assert
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn string_should_normalize_double_slashes() {
        // Arrange
        let non_normalized = "/foo//bar".to_string();

        // Act
        let segments = ToFixedSegments::to_segments(&non_normalized);

        // Assert
        assert_eq!(segments.len(), 2);
    }
}

#[cfg(test)]
mod to_parameterized_segment_tests {
    use crate::to_segments::ToParameterizedSegments;

    #[test]
    fn str_should_normalize_double_slashes() {
        // Arrange
        let non_normalized = "/foo//bar";

        // Act
        let segments = ToParameterizedSegments::to_segments(&non_normalized);

        // Assert
        assert_eq!(segments.len(), 2);
    }

    #[test]
    fn string_should_normalize_double_slashes() {
        // Arrange
        let non_normalized = "/foo//bar".to_string();

        // Act
        let segments = ToParameterizedSegments::to_segments(&non_normalized);

        // Assert
        assert_eq!(segments.len(), 2);
    }
}
