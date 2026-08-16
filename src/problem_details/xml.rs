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

    /// The XML namespace of a problem details document.
    pub const NAMESPACE: &'static str = "urn:ietf:rfc:7807";
}

impl<Ext> XmlProblemDetails<Ext>
where
    Ext: serde::Serialize,
{
    /// Write this problem details to an XML string suitable for a response body.
    pub fn to_body_string(&self) -> Result<String, XmlError> {
        let root = WithNamespace {
            xmlns: Self::NAMESPACE,
            problem: &self.0,
        };

        let xml =
            quick_xml::se::to_string_with_root("problem", &root).map_err(XmlError::serialization)?;
        let xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>{xml}"#);

        Ok(xml)
    }
}

/// Declares the problem details namespace on the root element.
///
/// quick-xml writes a field named `@xmlns` as an attribute of the enclosing
/// element, and flattening the problem details behind it leaves its members as
/// child elements of `<problem>`, exactly as they were without the wrapper.
#[derive(serde::Serialize)]
struct WithNamespace<'a, Ext> {
    #[serde(rename = "@xmlns")]
    xmlns: &'static str,

    #[serde(flatten)]
    problem: &'a ProblemDetails<Ext>,
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

/// An error that occurred while writing a
/// [`XmlProblemDetails`] to a response body.
///
/// The underlying error is deliberately not exposed as a concrete type, so that
/// the XML backend stays an implementation detail. Use [`XmlError::get_ref`] or
/// [`std::error::Error::source`] to inspect it.
#[derive(Clone)]
pub struct XmlError {
    kind: ErrorKind,
}

#[derive(Clone)]
enum ErrorKind {
    Serialization(quick_xml::SeError),
}

impl XmlError {
    pub(crate) fn serialization(err: quick_xml::SeError) -> Self {
        Self {
            kind: ErrorKind::Serialization(err),
        }
    }

    /// Returns a reference to the underlying error.
    pub fn get_ref(&self) -> &(dyn std::error::Error + 'static) {
        match &self.kind {
            ErrorKind::Serialization(err) => err,
        }
    }
}

impl std::fmt::Debug for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Done by hand to hide the private ErrorKind enum
        f.debug_tuple("XmlError").field(&self.get_ref()).finish()
    }
}

impl std::fmt::Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not write body: {}", self.get_ref())
    }
}

impl std::error::Error for XmlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.get_ref())
    }
}
