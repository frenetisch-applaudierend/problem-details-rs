use crate::ProblemDetails;

/// ProblemDetails that is encoded to JSON when
/// used with web framework integrations.
///
/// # Example
///
/// ```rust
/// use http::StatusCode;
/// use problem_details::{JsonProblemDetails, ProblemDetails};
///
/// async fn handler() -> JsonProblemDetails {
///     ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
///         .with_detail("short and stout")
///         .into()
/// }
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JsonProblemDetails<Ext = ()>(pub(crate) ProblemDetails<Ext>);

impl<Ext> JsonProblemDetails<Ext> {
    /// The HTTP content type for a json problem details.
    pub const CONTENT_TYPE: &'static str = "application/problem+json";
}

impl<Ext> JsonProblemDetails<Ext>
where
    Ext: serde::Serialize,
{
    /// Write this problem details to an JSON string suitable for a response body.
    pub fn to_body_string(&self) -> Result<String, JsonError> {
        serde_json::to_string(&self.0).map_err(JsonError::Serialization)
    }
}

impl<Ext> From<ProblemDetails<Ext>> for JsonProblemDetails<Ext> {
    fn from(value: ProblemDetails<Ext>) -> Self {
        Self(value)
    }
}

impl<Ext> From<JsonProblemDetails<Ext>> for ProblemDetails<Ext> {
    fn from(value: JsonProblemDetails<Ext>) -> Self {
        value.0
    }
}
impl<Ext> std::fmt::Display for JsonProblemDetails<Ext> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Ext> std::error::Error for JsonProblemDetails<Ext> where Ext: std::fmt::Debug {}

#[derive(Debug)]
pub enum JsonError {
    Serialization(serde_json::Error),
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Could not write body: {}",
            match self {
                Self::Serialization(err) => err,
            }
        )
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(err) => Some(err),
        }
    }
}
