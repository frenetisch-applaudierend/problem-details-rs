use http::Uri;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemType(Uri);

impl std::convert::From<Uri> for ProblemType {
    fn from(value: Uri) -> Self {
        ProblemType(value)
    }
}

impl std::default::Default for ProblemType {
    fn default() -> Self {
        ProblemType(Uri::from_static("about:blank"))
    }
}

impl std::ops::Deref for ProblemType {
    type Target = Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<Uri> for ProblemType {
    fn borrow(&self) -> &Uri {
        &self.0
    }
}
