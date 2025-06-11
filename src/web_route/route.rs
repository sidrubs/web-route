use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{to_segments::ToFixedSegments, web_route::segment::WebSegment};

/// Defines a route structure that can be safely joined, no matter the
/// leading/trailing slash configuration or operating system.
#[derive(Clone, PartialEq)]
pub struct WebRoute {
    segments: Vec<WebSegment>,
}

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
        Self {
            segments: route.to_segments(),
        }
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
        Self {
            segments: [self.segments.clone(), route.to_segments()].concat(),
        }
    }

    pub(crate) fn segments(&self) -> Vec<WebSegment> {
        self.segments.clone()
    }
}

impl fmt::Display for WebRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let evaluated_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_evaluated())
            .collect::<Vec<_>>();

        write!(f, "/{}", evaluated_segments.join("/"))
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
