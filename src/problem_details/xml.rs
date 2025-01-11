use crate::ProblemDetails;

/// ProblemDetails that is encoded to XML when
/// used with web framework integrations.
///
/// # Example
///
/// ```rust
/// use http::StatusCode;
/// use problem_details::{XmlProblemDetails, ProblemDetails};
///
/// async fn handler() -> XmlProblemDetails {
///     ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
///         .with_detail("short and stout")
///         .into()
/// }
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XmlProblemDetails<Ext = ()>(pub(crate) ProblemDetails<Ext>);

impl<Ext> XmlProblemDetails<Ext> {
    /// The HTTP content type for a xml problem details.
    pub const CONTENT_TYPE: &'static str = "application/problem+xml";
}

impl<Ext> XmlProblemDetails<Ext>
where
    Ext: serde::Serialize,
{
    /// Write this problem details to an XML string suitable for a response body.
    pub fn to_body_string(&self) -> Result<String, XmlError> {
        let xml = quick_xml::se::to_string_with_root("problem", &self.0)
            .map_err(|e| XmlError::Serialization(e))?;
        let xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, xml);

        Ok(xml)
    }
}

impl<Ext> From<ProblemDetails<Ext>> for XmlProblemDetails<Ext> {
    fn from(value: ProblemDetails<Ext>) -> Self {
        Self(value)
    }
}

impl<Ext> From<XmlProblemDetails<Ext>> for ProblemDetails<Ext> {
    fn from(value: XmlProblemDetails<Ext>) -> Self {
        value.0
    }
}
impl<Ext> std::fmt::Display for XmlProblemDetails<Ext> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Ext> std::error::Error for XmlProblemDetails<Ext> where Ext: std::fmt::Debug {}

#[derive(Clone, Debug)]
pub enum XmlError {
    Serialization(quick_xml::SeError),
}

impl std::fmt::Display for XmlError {
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

impl std::error::Error for XmlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(err) => Some(err),
        }
    }
}
