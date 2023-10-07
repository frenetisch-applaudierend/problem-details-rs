use http::{StatusCode, Uri};

use crate::ProblemType;

pub struct ProblemDetails {
    r#type: Option<ProblemType>,
    status: Option<StatusCode>,
    title: Option<String>,
    detail: Option<String>,
    instance: Option<Uri>,
}

impl ProblemDetails {
    pub fn from_status(status: StatusCode) -> Self {
        ProblemDetails {
            r#type: None,
            status: Some(status),
            title: status.canonical_reason().map(ToOwned::to_owned),
            detail: None,
            instance: None,
        }
    }

    pub fn from_type(r#type: ProblemType) -> Self {
        ProblemDetails {
            r#type: Some(r#type),
            status: None,
            title: None,
            detail: None,
            instance: None,
        }
    }

    pub fn r#type(&self) -> Option<&ProblemType> {
        self.r#type.as_ref()
    }

    pub fn set_type(&mut self, r#type: Option<impl Into<ProblemType>>) {
        self.r#type = r#type.map(Into::into);
    }

    pub fn with_type(mut self, r#type: impl Into<ProblemType>) -> Self {
        self.set_type(Some(r#type));
        self
    }

    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    pub fn set_status(&mut self, status: Option<StatusCode>) {
        self.status = status;
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.set_status(Some(status));
        self
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(|t| t.as_str())
    }

    pub fn set_title(&mut self, title: Option<impl Into<String>>) {
        self.title = title.map(Into::into);
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.set_title(Some(title));
        self
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_ref().map(|t| t.as_str())
    }

    pub fn set_detail(&mut self, detail: Option<impl Into<String>>) {
        self.detail = detail.map(Into::into);
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.set_detail(Some(detail));
        self
    }

    pub fn instance(&self) -> Option<&Uri> {
        self.instance.as_ref()
    }

    pub fn set_instance(&mut self, instance: Option<impl Into<Uri>>) {
        self.instance = instance.map(Into::into);
    }

    pub fn with_instance(mut self, instance: impl Into<Uri>) -> Self {
        self.set_instance(Some(instance));
        self
    }
}

#[cfg(test)]
mod tests {
    use http::{StatusCode, Uri};

    use crate::{ProblemDetails, ProblemType};

    #[test]
    fn configure_from_status() {
        let details = ProblemDetails::from_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_type(Uri::from_static("test:uri"))
            .with_instance(Uri::from_static("test:instance"));

        assert_eq!(
            details.r#type(),
            Some(&ProblemType::from(Uri::from_static("test:uri")))
        );
    }
}
