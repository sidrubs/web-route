/// Represents an individual segment of a route (i.e. the bit between the `/`).
#[derive(Debug, Clone, PartialEq)]
pub struct WebSegment(String);

impl WebSegment {
    /// Returns the value of the segment.
    pub(crate) fn to_evaluated(&self) -> String {
        self.0.clone()
    }
}

impl From<&str> for WebSegment {
    fn from(segment: &str) -> Self {
        Self(segment.to_owned())
    }
}
