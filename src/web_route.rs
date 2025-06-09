use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    error::WebRouteError, segment::Segment, to_segments::ToSegments, utils::struct_to_map,
};

#[derive(Clone, PartialEq)]
pub struct WebRoute {
    segments: Vec<Segment>,
}

impl WebRoute {
    /// Creates a new [`WebRoute`].
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::WebRoute;
    ///
    /// let route = WebRoute::new("/some/route/{param}");
    /// ```
    pub fn new<R: ToSegments>(route: R) -> Self {
        Self {
            segments: route.to_segments(),
        }
    }

    /// Joins a route onto an existing [`WebRoute`] returning the joined route.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::WebRoute;
    ///
    /// let route = WebRoute::new("/some/route/{param}");
    /// let nested_route = WebRoute::new("/a/nested/route");
    /// let joined_route = route.join(&nested_route);
    ///
    /// assert_eq!(joined_route, route.join("/a/nested/route"))
    /// ```
    pub fn join<R: ToSegments>(&self, route: R) -> Self {
        Self {
            segments: [self.segments.clone(), route.to_segments()].concat(),
        }
    }

    /// Returns the route in its "templated" representation so that it can be
    /// used in web server route definitions.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::WebRoute;
    ///
    /// let route = WebRoute::new("/some/route/{param}");
    /// let template_route = route.as_template_route();
    ///
    /// assert_eq!(template_route, "/some/route/{param}")
    /// ```
    pub fn as_template_route(&self) -> String {
        let template_segments = self
            .segments
            .iter()
            .map(Segment::to_template)
            .collect::<Vec<_>>();

        format!("/{}", template_segments.join("/"))
    }

    /// Attempts to populate the parameters of the route with their `values`.
    ///
    /// `values` needs to implement `serde::Serialize` and be of an "Object"
    /// style (with key-value pairs).
    ///
    /// This would be used when making a request to an endpoint represented by
    /// the route.
    ///
    /// # Errors
    ///
    /// - [`WebRouteError::UnpopulatedParam`] if no matching entry was found in
    ///   `values` for a particular parameter.
    /// - [`WebRouteError::InvalidValue`] if `values` does not contain
    ///   key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::WebRoute;
    ///
    /// #[derive(serde::Serialize)]
    /// struct RouteParams {
    ///     param: String,
    /// }
    ///
    /// let route = WebRoute::new("/some/route/{param}");
    /// let populated_route = route
    ///     .as_populated_route(&RouteParams {
    ///         param: "value".to_owned(),
    ///     })
    ///     .unwrap();
    ///     
    /// assert_eq!(populated_route, "/some/route/value")
    /// ```
    pub fn as_populated_route<V: serde::Serialize>(
        &self,
        values: &V,
    ) -> Result<String, WebRouteError> {
        let values = struct_to_map(values).ok_or(WebRouteError::InvalidValue)?;

        let populated_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_populated(&values))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(format!("/{}", populated_segments.join("/")))
    }

    pub(crate) fn segments(&self) -> Vec<Segment> {
        self.segments.clone()
    }
}

impl fmt::Debug for WebRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WebRoute")
            .field(&self.as_template_route())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl Serialize for WebRoute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.as_template_route();
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
