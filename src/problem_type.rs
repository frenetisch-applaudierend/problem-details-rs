use http::Uri;

/// A type that represents a problem type URI.
///
/// This type is mostly a wrapper around `http::Uri`. It implements
/// `std::default::Default` to return `about:blank` as the default problem type.
///
/// # Creating a problem type
///
/// You should rarely need to create a [`ProblemType`] manually. Instead, you can
/// just create an [`Uri`](http::Uri) and pass that e.g. to [`ProblemDetails::with_type`](crate::ProblemDetails::with_type).
///
/// In case you do need to create a [`ProblemType`] manually, you can use
/// the [`From`](std::convert::From) trait to convert a given [`Uri`](http::Uri),
/// or [`Default::default()`] to create a default URI.
///
/// ```rust
/// use http::Uri;
/// use problem_details::ProblemType;
///
/// // Create a problem type from a URI
/// let uri = Uri::from_static("https://example.com/problem");
/// let problem_type = ProblemType::from(uri);
/// assert_eq!(problem_type.to_string(), "https://example.com/problem");
///
/// // Create a default problem type
/// let default_type = ProblemType::default();
/// assert_eq!(default_type.to_string(), "about:blank");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ProblemType(String);

impl ProblemType {
    /// Creates a problem type from any string without validating it.
    ///
    /// The caller is responsible for passing a valid URI reference as defined by
    /// [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986). Absolute URIs are recommended.
    #[must_use]
    pub fn unchecked(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the problem type as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::default::Default for ProblemType {
    fn default() -> Self {
        Self::unchecked("about:blank")
    }
}

impl std::fmt::Display for ProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::convert::From<Uri> for ProblemType {
    fn from(value: Uri) -> Self {
        Self(value.to_string())
    }
}

impl std::convert::From<&Uri> for ProblemType {
    fn from(value: &Uri) -> Self {
        Self(value.to_string())
    }
}

impl std::convert::TryFrom<ProblemType> for Uri {
    type Error = http::uri::InvalidUri;

    fn try_from(value: ProblemType) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl std::convert::TryFrom<&ProblemType> for Uri {
    type Error = http::uri::InvalidUri;

    fn try_from(value: &ProblemType) -> Result<Self, Self::Error> {
        value.0.parse()
    }
}

impl std::convert::AsRef<str> for ProblemType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
