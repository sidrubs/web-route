use std::fmt;

use crate::{
    WebRoute, error::WebRouteError, parameterized_route::segment::ParameterizedSegment,
    to_segments::ToParameterizedSegments, utils::struct_to_map,
};

/// Defines a route structure that can be used to define routes for a webserver.
///
/// Its templated sections can be easily populated to create a [`WebRoute`]
/// which can be used to make requests against the webserver routes that the
/// [`ParameterizedRoute`] was used to define.
#[derive(Clone, PartialEq)]
pub struct ParameterizedRoute {
    segments: Vec<ParameterizedSegment>,
}

impl ParameterizedRoute {
    /// Creates a new [`ParameterizedRoute`].
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::ParameterizedRoute;
    ///
    /// let route = ParameterizedRoute::new("/some/route/{param}");
    /// ```
    pub fn new<R: ToParameterizedSegments>(route: R) -> Self {
        Self {
            segments: route.to_segments(),
        }
    }

    /// Joins a route onto an existing [`ParameterizedRoute`] returning the
    /// joined route.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::ParameterizedRoute;
    ///
    /// let route = ParameterizedRoute::new("/some/route/{param}");
    /// let nested_route = ParameterizedRoute::new("/a/nested/route");
    /// let joined_route = route.join(&nested_route);
    ///
    /// assert_eq!(joined_route, route.join("/a/nested/route"))
    /// ```
    pub fn join<R: ToParameterizedSegments>(&self, route: R) -> Self {
        Self {
            segments: [self.segments.clone(), route.to_segments()].concat(),
        }
    }

    /// Attempts to populate the parameters of the route with their `values` and
    /// returns a [`WebRoute`].
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
    /// - [`WebRouteError::InvalidValue`] if `values` does not contain key-value
    ///   pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use web_route::ParameterizedRoute;
    ///
    /// #[derive(serde::Serialize)]
    /// struct RouteParams {
    ///     param: String,
    /// }
    ///
    /// let parameterized_route = ParameterizedRoute::new("/some/route/{param}");
    /// let web_route = parameterized_route
    ///     .to_web_route(&RouteParams {
    ///         param: "value".to_owned(),
    ///     })
    ///     .unwrap();
    ///     
    /// assert_eq!(&web_route.to_string(), "/some/route/value")
    /// ```
    pub fn to_web_route<V: serde::Serialize>(&self, values: &V) -> Result<WebRoute, WebRouteError> {
        let values = struct_to_map(values).ok_or(WebRouteError::InvalidValue)?;

        let populated_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_populated(&values))
            .collect::<Result<Vec<_>, _>>()?;

        let web_route = WebRoute::new(format!("/{}", populated_segments.join("/")));

        Ok(web_route)
    }

    pub(crate) fn segments(&self) -> Vec<ParameterizedSegment> {
        self.segments.clone()
    }
}

impl fmt::Display for ParameterizedRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let template_segments = self
            .segments
            .iter()
            .map(ParameterizedSegment::to_template)
            .collect::<Vec<_>>();

        write!(f, "/{}", template_segments.join("/"))
    }
}

impl fmt::Debug for ParameterizedRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ParameterizedRoute")
            .field(&self.to_string())
            .finish()
    }
}
