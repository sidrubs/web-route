use std::{fmt, ops};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{to_segments::ToFixedSegments, web_route::segment::WebSegment};

/// Defines a route structure that can be safely joined, no matter the
/// leading/trailing slash configuration or operating system.
#[derive(Clone, PartialEq)]
pub struct WebRoute(String);

impl WebRoute {
    /// Creates a new [`WebRoute`].
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::WebRoute;
    ///
    /// let route = WebRoute::new("/some/route");
    /// ```
    pub fn new<R: ToFixedSegments>(route: R) -> Self {
        let segments = route.to_segments();

        Self(evaluate_segments(segments))
    }

    /// Joins a route onto an existing [`WebRoute`] returning the joined
    /// route.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::WebRoute;
    ///
    /// let route = WebRoute::new("/some/route/");
    /// let nested_route = WebRoute::new("/a/nested/route");
    /// let joined_route = route.join(&nested_route);
    ///
    /// assert_eq!(&joined_route.to_string(), "/some/route/a/nested/route")
    /// ```
    pub fn join<R: ToFixedSegments>(&self, route: R) -> Self {
        let joined_segments = [self.to_segments(), route.to_segments()].concat();

        Self(evaluate_segments(joined_segments))
    }

    pub(crate) fn to_segments(&self) -> Vec<WebSegment> {
        ToFixedSegments::to_segments(&self.0)
    }
}

impl fmt::Display for WebRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for WebRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WebRoute").field(&self.to_string()).finish()
    }
}

#[cfg(feature = "serde")]
impl Serialize for WebRoute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for WebRoute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(WebRoute::new(s))
    }
}

/// Allows one to deref for usage with external crates. Makes for neater code.
impl ops::Deref for WebRoute {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for WebRoute {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "fake")]
impl fake::Dummy<fake::Faker> for WebRoute {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(config: &fake::Faker, rng: &mut R) -> Self {
        use fake::Fake;

        let segments: Vec<WebSegment> = config.fake_with_rng(rng);
        Self::new(segments)
    }
}

/// Convert `segments` into their normalized [`String`] route representation.
fn evaluate_segments(segments: Vec<WebSegment>) -> String {
    let evaluated_segments = segments
        .iter()
        .map(|segment| segment.to_evaluated())
        .collect::<Vec<_>>();

    format!("/{}", evaluated_segments.join("/"))
}
