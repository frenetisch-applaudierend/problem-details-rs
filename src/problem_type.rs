use http::Uri;

/// A type that represents a problem type uri.
///
/// This type is mostly a wrapper around `http::Uri`. It implements
/// `std::default::Default` to return `about:blank` as the default problem type.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProblemType(#[cfg_attr(feature = "serde", serde(with = "crate::serde::uri"))] Uri);

impl std::default::Default for ProblemType {
    fn default() -> Self {
        ProblemType(Uri::from_static("about:blank"))
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
