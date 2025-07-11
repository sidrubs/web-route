use std::collections::HashMap;

use crate::{error::WebRouteError, web_route::segment::WebSegment};

/// Represents an individual segment of a route (i.e. the bit between the `/`).
///
/// Handles converting it between the templated representation of the segment,
/// and the populated version.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterizedSegment {
    NamedParam(String),
    CatchallParam(String),
    Static(String),
}

impl ParameterizedSegment {
    /// Converts the [`Segment`] into its "templated" representation so that it
    /// can be used in route definitions.
    pub(crate) fn to_template(&self) -> String {
        match self {
            ParameterizedSegment::NamedParam(named_param) => format!("{{{named_param}}}"),
            ParameterizedSegment::CatchallParam(catchall_param) => format!("{{*{catchall_param}}}"),
            ParameterizedSegment::Static(value) => value.to_owned(),
        }
    }

    /// Attempts to populate a parameter based [`Segment`] with a value in the
    /// `param_value_map`. If the [`Segment`] is static, it internal value will
    /// be returned.
    ///
    /// # Errors
    ///
    /// A [`WebRouteError::UnpopulatedParam`] if no matching entry was found in
    /// the `param_value_map`.
    pub(crate) fn to_populated(
        &self,
        param_value_map: &HashMap<String, String>,
    ) -> Result<String, WebRouteError> {
        let populated = match self {
            ParameterizedSegment::NamedParam(param) => param_value_map
                .get(param)
                .ok_or(WebRouteError::UnpopulatedParam(param.to_owned()))?
                .to_owned(),
            ParameterizedSegment::CatchallParam(param) => param_value_map
                .get(param)
                .ok_or(WebRouteError::UnpopulatedParam(param.to_owned()))?
                .to_owned(),
            ParameterizedSegment::Static(value) => value.to_owned(),
        };

        Ok(populated)
    }
}

/// It is often a path of insecure traversals if there are two consecutive slashes in a path. Making an empty [`ParameterizedSegment`] impossible to create removes the chance of consecutive slashes.
impl TryFrom<&str> for ParameterizedSegment {
    type Error = ();

    fn try_from(segment: &str) -> Result<Self, Self::Error> {
        let segment = segment.trim();

        if segment.is_empty() {
            return Err(());
        }

        Ok(if segment.starts_with("{*") && segment.ends_with('}') {
            let param = segment.trim_start_matches("{*").trim_end_matches('}');
            ParameterizedSegment::CatchallParam(param.to_string())
        } else if segment.starts_with('{') && segment.ends_with('}') {
            let param = segment.trim_start_matches('{').trim_end_matches('}');
            ParameterizedSegment::NamedParam(param.to_string())
        } else {
            ParameterizedSegment::Static(segment.to_string())
        })
    }
}

impl From<WebSegment> for ParameterizedSegment {
    fn from(value: WebSegment) -> Self {
        Self::Static(value.to_evaluated())
    }
}

#[cfg(test)]
mod segment_tests {
    use super::*;

    mod to_template {
        use super::*;

        #[test]
        fn should_correctly_template_named_parameter() {
            // Arrange
            let segment = ParameterizedSegment::NamedParam("named_param".to_owned());

            // Act
            let template = segment.to_template();

            // Assert
            assert_eq!(template, "{named_param}");
        }

        #[test]
        fn should_correctly_template_catchall_parameter() {
            // Arrange
            let segment = ParameterizedSegment::CatchallParam("catchall_param".to_owned());

            // Act
            let template = segment.to_template();

            // Assert
            assert_eq!(template, "{*catchall_param}");
        }

        #[test]
        fn should_correctly_template_static_segment() {
            // Arrange
            let segment = ParameterizedSegment::Static("static".to_owned());

            // Act
            let template = segment.to_template();

            // Assert
            assert_eq!(template, "static");
        }
    }

    mod from {
        use super::*;

        #[test]
        fn should_parse_named_parameter() {
            // Act
            let segment = ParameterizedSegment::try_from("{named_param}").unwrap();

            // Assert
            assert!(
                matches!(segment, ParameterizedSegment::NamedParam(param) if param == "named_param")
            );
        }

        #[test]
        fn should_parse_catchall_parameter() {
            // Act
            let segment = ParameterizedSegment::try_from("{*catchall_param}").unwrap();

            // Assert
            assert!(
                matches!(segment, ParameterizedSegment::CatchallParam(param) if param == "catchall_param")
            );
        }

        #[test]
        fn should_parse_static_segment() {
            // Act
            let segment = ParameterizedSegment::try_from("static").unwrap();

            // Assert
            assert!(matches!(segment, ParameterizedSegment::Static(value) if value == "static"));
        }

        #[test]
        fn should_not_parse_empty_segment() {
            // Act
            let res = ParameterizedSegment::try_from("");

            // Assert
            assert!(res.is_err());
        }
    }
}
