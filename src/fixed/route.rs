use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{fixed::segment::FixedSegment, to_segments::ToFixedSegments};

#[derive(Clone, PartialEq)]
pub struct FixedRoute {
    segments: Vec<FixedSegment>,
}

impl FixedRoute {
    /// Creates a new [`FixedRoute`].
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::FixedRoute;
    ///
    /// let route = FixedRoute::new("/some/route");
    /// ```
    pub fn new<R: ToFixedSegments>(route: R) -> Self {
        Self {
            segments: route.to_segments(),
        }
    }

    /// Joins a route onto an existing [`FixedRoute`] returning the joined route.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::FixedRoute;
    ///
    /// let route = FixedRoute::new("/some/route/");
    /// let nested_route = FixedRoute::new("/a/nested/route");
    /// let joined_route = route.join(&nested_route);
    ///
    /// assert_eq!(joined_route, route.join("/a/nested/route"))
    /// ```
    pub fn join<R: ToFixedSegments>(&self, route: R) -> Self {
        Self {
            segments: [self.segments.clone(), route.to_segments()].concat(),
        }
    }

    /// Reconstructs a normalized route. Starts with a `/` and has no trailing slash.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::FixedRoute;
    ///
    /// let route = FixedRoute::new("some/route/");
    /// let evaluated_route = route.as_evaluated_route();
    ///     
    /// assert_eq!(evaluated_route, "/some/route")
    /// ```
    pub fn as_evaluated_route(&self) -> String {
        let evaluated_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_evaluated())
            .collect::<Vec<_>>();

        format!("/{}", evaluated_segments.join("/"))
    }

    pub(crate) fn segments(&self) -> Vec<FixedSegment> {
        self.segments.clone()
    }
}

impl fmt::Debug for FixedRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FixedRoute")
            .field(&self.as_evaluated_route())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl Serialize for FixedRoute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.as_evaluated_route();
        serializer.serialize_str(&s)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for FixedRoute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(FixedRoute::new(s))
    }
}
