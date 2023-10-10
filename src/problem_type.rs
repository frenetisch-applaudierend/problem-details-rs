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
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProblemType(#[cfg_attr(feature = "serde", serde(with = "crate::serde::uri"))] Uri);

impl std::default::Default for ProblemType {
    fn default() -> Self {
        Self(Uri::from_static("about:blank"))
    }
}

impl std::fmt::Display for ProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::convert::From<Uri> for ProblemType {
    fn from(value: Uri) -> Self {
        ProblemType(value)
    }
}

impl std::convert::From<ProblemType> for Uri {
    fn from(value: ProblemType) -> Self {
        value.0
    }
}

impl std::ops::Deref for ProblemType {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ProblemType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::borrow::Borrow<Uri> for ProblemType {
    fn borrow(&self) -> &Uri {
        &self.0
    }
}

impl std::borrow::BorrowMut<Uri> for ProblemType {
    fn borrow_mut(&mut self) -> &mut Uri {
        &mut self.0
    }
}

impl std::convert::AsRef<Uri> for ProblemType {
    fn as_ref(&self) -> &Uri {
        &self.0
    }
}

impl std::convert::AsMut<Uri> for ProblemType {
    fn as_mut(&mut self) -> &mut Uri {
        &mut self.0
    }
}
