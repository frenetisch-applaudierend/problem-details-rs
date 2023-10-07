use http::{StatusCode, Uri};

use crate::ProblemType;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProblemDetails {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub r#type: Option<ProblemType>,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::status::opt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub status: Option<StatusCode>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub detail: Option<String>,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::uri::opt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub instance: Option<Uri>,
}

impl ProblemDetails {
    pub fn new() -> Self {
        ProblemDetails {
            r#type: None,
            status: None,
            title: None,
            detail: None,
            instance: None,
        }
    }

    pub fn from_status(status: StatusCode) -> Self {
        ProblemDetails {
            r#type: None,
            status: Some(status),
            title: status.canonical_reason().map(ToOwned::to_owned),
            detail: None,
            instance: None,
        }
    }

    pub fn with_type(mut self, r#type: impl Into<ProblemType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<StatusCode>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_instance(mut self, instance: impl Into<Uri>) -> Self {
        self.instance = Some(instance.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use http::{StatusCode, Uri};

    use crate::{ProblemDetails, ProblemType};

    #[test]
    fn from_status() {
        let details = ProblemDetails::from_status(StatusCode::NOT_FOUND);

        assert_eq!(
            details.r#type.unwrap_or_default(),
            ProblemType::from(Uri::from_static("about:blank"))
        );
        assert_eq!(details.status, Some(StatusCode::NOT_FOUND));
        assert_eq!(details.title, Some("Not Found".to_string()));
        assert_eq!(details.detail, None);
        assert_eq!(details.instance, None);
    }

    #[test]
    fn fully_configured() {
        let details = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_title("Test Title")
            .with_detail("Test Detail")
            .with_instance(Uri::from_static("test:instance"));

        assert_eq!(
            details.r#type,
            Some(ProblemType::from(Uri::from_static("test:type")))
        );
        assert_eq!(details.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
        assert_eq!(details.title, Some("Test Title".to_string()));
        assert_eq!(details.detail, Some("Test Detail".to_string()));
        assert_eq!(details.instance, Some(Uri::from_static("test:instance")));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_empty() {
        let empty = ProblemDetails::new();

        let serialized = serde_json::to_value(empty).unwrap();

        let expected = serde_json::json!({});

        assert_eq!(expected, serialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_filled() {
        let details = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_title("Test Title")
            .with_detail("Test Detail")
            .with_instance(Uri::from_static("test:instance"));

        let serialized = serde_json::to_value(details).unwrap();

        let expected = serde_json::json!({
            "type": "test:type",
            "status": 500,
            "title": "Test Title",
            "detail": "Test Detail",
            "instance": "test:instance"
        });

        assert_eq!(expected, serialized);
    }
}
