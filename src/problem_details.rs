use http::{StatusCode, Uri};

use crate::ProblemType;

/// A RFC 9457 / RFC 7807 problem details object.
///
/// # Creating problem details
///
/// You can create a new problem details from a given
/// status code using [`ProblemDetails::from_status_code`].
///
/// This will set the `status` field to the given status code,
/// the `title` field to the canonical reason phrase of the status code,
/// and the `type` field to none, which is equivalen to `about:blank`.
///
/// ```rust
/// use http::StatusCode;
/// use problem_details::ProblemDetails;
///
/// let details = ProblemDetails::from_status_code(StatusCode::NOT_FOUND);
///
/// assert_eq!(details.status, Some(StatusCode::NOT_FOUND));
/// assert_eq!(details.title, Some("Not Found".to_string()));
/// assert_eq!(details.r#type.unwrap_or_default(), problem_details::ProblemType::default());
/// ```
///
/// You can then use the builder pattern to add additional fields.
///
/// ```rust
/// use http::{StatusCode, Uri};
/// use problem_details::ProblemDetails;
///
/// let details = ProblemDetails::from_status_code(StatusCode::NOT_FOUND)
///    .with_type(Uri::from_static("example:type"))
///    .with_title("There is something wrong");
///
/// assert_eq!(details.status, Some(StatusCode::NOT_FOUND));
/// assert_eq!(details.title, Some("There is something wrong".to_string()));
/// assert_eq!(details.r#type.unwrap_or_default(), Uri::from_static("example:type").into());
/// ```
///
/// You can also create a new problem details object using [`ProblemDetails::new`].
///
/// ```rust
/// use http::Uri;
/// use problem_details::ProblemDetails;
///
/// let details = ProblemDetails::new()
///   .with_type(Uri::from_static("example:type"))
///   .with_title("There is something wrong");
///
/// assert_eq!(details.status, None);
/// assert_eq!(details.title, Some("There is something wrong".to_string()));
/// assert_eq!(details.r#type.unwrap_or_default(), Uri::from_static("example:type").into());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProblemDetails<Ext = ()> {
    /// An optional uri describing the problem type.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-type]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub r#type: Option<ProblemType>,

    /// An optional status code for this problem.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-status]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::status::opt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub status: Option<StatusCode>,

    /// An optional human-readable title for this problem.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-title]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub title: Option<String>,

    /// An optional human-readable description of this problem.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-detail]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub detail: Option<String>,

    /// An optional uri identifying the specific instance of this problem.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-instance]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::uri::opt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub instance: Option<Uri>,

    /// An object containing extensions to this problem details object.
    ///
    /// Note that the extensions will be flattened into the resulting problem details
    /// representation.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-extension-members]() for more information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub extensions: Ext,
}

impl ProblemDetails<()> {
    /// Creates a new empty problem details object.
    pub fn new() -> Self {
        Self {
            r#type: None,
            status: None,
            title: None,
            detail: None,
            instance: None,
            extensions: Default::default(),
        }
    }

    /// Creates a new problem details object from a given status code.
    ///
    /// This will set the `status` field to the given status code,
    /// the `title` field to the canonical reason phrase of the status code,
    /// and the `type` field to none, which is equivalent to `about:blank`.
    pub fn from_status_code(status: StatusCode) -> Self {
        Self {
            r#type: None,
            status: Some(status),
            title: status.canonical_reason().map(ToOwned::to_owned),
            detail: None,
            instance: None,
            extensions: Default::default(),
        }
    }
}

impl<Ext> ProblemDetails<Ext> {
    /// Builder-style method that sets the `type` field of this problem details object.
    pub fn with_type(mut self, r#type: impl Into<ProblemType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Builder-style method that sets the `status` field of this problem details object.
    pub fn with_status(mut self, status: impl Into<StatusCode>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Builder-style method that sets the `title` field of this problem details object.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Builder-style method that sets the `detail` field of this problem details object.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Builder-style method that sets the `instance` field of this problem details object.
    pub fn with_instance(mut self, instance: impl Into<Uri>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// Builder style method that sets the `extensions` field of this probelm details object.
    pub fn with_extensions<NewExt>(self, extensions: NewExt) -> ProblemDetails<NewExt> {
        ProblemDetails::<NewExt> {
            r#type: self.r#type,
            status: self.status,
            title: self.title,
            detail: self.detail,
            instance: self.instance,
            extensions,
        }
    }
}

impl<Ext> std::fmt::Display for ProblemDetails<Ext> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default_type = ProblemType::default();
        let r#type = self.r#type.as_ref().unwrap_or(&default_type);
        write!(f, "[{type}")?;

        if let Some(status) = self.status {
            write!(f, " {}]", status.as_u16())?;
        } else {
            write!(f, "]")?;
        }

        let title = self
            .title
            .as_deref()
            .or(self.status.as_ref().and_then(StatusCode::canonical_reason));

        if let Some(title) = title {
            write!(f, " {title}")?;
        }

        if let Some(detail) = self.detail.as_ref() {
            if title.is_some() {
                write!(f, ":")?;
            }

            write!(f, " {detail}")?;
        }

        // if let Some()
        Ok(())
    }
}

impl<Ext> std::error::Error for ProblemDetails<Ext> where Ext: std::fmt::Debug {}

#[cfg(test)]
mod tests {
    use http::{StatusCode, Uri};
    use serde_json::json;

    use crate::{ProblemDetails, ProblemType};

    #[test]
    fn from_status() {
        let details = ProblemDetails::from_status_code(StatusCode::NOT_FOUND);

        assert_eq!(
            details.r#type.unwrap_or_default(),
            ProblemType::from(Uri::from_static("about:blank"))
        );
        assert_eq!(details.status, Some(StatusCode::NOT_FOUND));
        assert_eq!(details.title, Some("Not Found".to_string()));
        assert_eq!(details.detail, None);
        assert_eq!(details.instance, None);
        assert_eq!(details.extensions, ());
    }

    #[test]
    fn fully_configured() {
        #[derive(Debug, PartialEq, Eq)]
        struct Extensions {
            foo: String,
            bar: u32,
        }

        let details = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_title("Test Title")
            .with_detail("Test Detail")
            .with_instance(Uri::from_static("test:instance"))
            .with_extensions(Extensions {
                foo: "Foo".to_string(),
                bar: 42,
            });

        assert_eq!(
            details.r#type,
            Some(ProblemType::from(Uri::from_static("test:type")))
        );
        assert_eq!(details.status, Some(StatusCode::INTERNAL_SERVER_ERROR));
        assert_eq!(details.title, Some("Test Title".to_string()));
        assert_eq!(details.detail, Some("Test Detail".to_string()));
        assert_eq!(details.instance, Some(Uri::from_static("test:instance")));
        assert_eq!(
            details.extensions,
            Extensions {
                foo: "Foo".to_string(),
                bar: 42
            }
        );
    }

    #[test]
    fn to_string() {
        let empty = ProblemDetails::new();

        let type_only = ProblemDetails::new().with_type(Uri::from_static("test:type"));
        let status_only = ProblemDetails::new().with_status(StatusCode::NOT_FOUND);
        let title_only = ProblemDetails::new().with_title("Test Title");
        let detail_only = ProblemDetails::new().with_detail("Test Detail");

        let type_status = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::NOT_FOUND);
        let type_title = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_title("Test Title");
        let type_detail = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_detail("Test Detail");
        let status_title = ProblemDetails::new()
            .with_status(StatusCode::NOT_FOUND)
            .with_title("Test Title");
        let status_detail = ProblemDetails::new()
            .with_status(StatusCode::NOT_FOUND)
            .with_detail("Test Detail");
        let title_detail = ProblemDetails::new()
            .with_title("Test Title")
            .with_detail("Test Detail");

        let type_status_title = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::NOT_FOUND)
            .with_title("Test Title");
        let type_status_detail = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::NOT_FOUND)
            .with_detail("Test Detail");
        let type_title_detail = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_title("Test Title")
            .with_detail("Test Detail");
        let status_title_detail = ProblemDetails::new()
            .with_status(StatusCode::NOT_FOUND)
            .with_title("Test Title")
            .with_detail("Test Detail");

        let full = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::NOT_FOUND)
            .with_title("Test Title")
            .with_detail("Test Detail");

        assert_eq!("[about:blank]", empty.to_string());

        assert_eq!("[test:type]", type_only.to_string());
        assert_eq!("[about:blank 404] Not Found", status_only.to_string());
        assert_eq!("[about:blank] Test Title", title_only.to_string());
        assert_eq!("[about:blank] Test Detail", detail_only.to_string());

        assert_eq!("[test:type 404] Not Found", type_status.to_string());
        assert_eq!("[test:type] Test Title", type_title.to_string());
        assert_eq!("[test:type] Test Detail", type_detail.to_string());
        assert_eq!("[about:blank 404] Test Title", status_title.to_string());
        assert_eq!(
            "[about:blank 404] Not Found: Test Detail",
            status_detail.to_string()
        );
        assert_eq!(
            "[about:blank] Test Title: Test Detail",
            title_detail.to_string()
        );

        assert_eq!("[test:type 404] Test Title", type_status_title.to_string());
        assert_eq!(
            "[test:type 404] Not Found: Test Detail",
            type_status_detail.to_string()
        );
        assert_eq!(
            "[test:type] Test Title: Test Detail",
            type_title_detail.to_string()
        );
        assert_eq!(
            "[about:blank 404] Test Title: Test Detail",
            status_title_detail.to_string()
        );

        assert_eq!("[test:type 404] Test Title: Test Detail", full.to_string());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_empty() {
        let empty = ProblemDetails::new();

        let serialized = serde_json::to_value(empty).unwrap();

        let expected = json!({});

        assert_eq!(expected, serialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_filled() {
        #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        struct Extensions {
            foo: String,
            bar: u32,
        }

        let details = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_title("Test Title")
            .with_detail("Test Detail")
            .with_instance(Uri::from_static("test:instance"))
            .with_extensions(Extensions {
                foo: "Foo".to_string(),
                bar: 42,
            });

        let serialized = serde_json::to_value(details).unwrap();

        let expected = json!({
            "type": "test:type",
            "status": 500,
            "title": "Test Title",
            "detail": "Test Detail",
            "instance": "test:instance",
            "foo": "Foo",
            "bar": 42
        });

        assert_eq!(expected, serialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_empty() {
        let empty = json!({});

        let deserialized = serde_json::from_value(empty).unwrap();

        let expected = ProblemDetails::new();

        assert_eq!(expected, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialize_filled() {
        #[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        struct Extensions {
            foo: String,
            bar: u32,
        }

        let filled = json!({
            "type": "test:type",
            "status": 500,
            "title": "Test Title",
            "detail": "Test Detail",
            "instance": "test:instance",
            "foo": "Foo",
            "bar": 42
        });

        let deserialized: ProblemDetails<Extensions> = serde_json::from_value(filled).unwrap();

        let expected = ProblemDetails::new()
            .with_type(Uri::from_static("test:type"))
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
            .with_title("Test Title")
            .with_detail("Test Detail")
            .with_instance(Uri::from_static("test:instance"))
            .with_extensions(Extensions {
                foo: "Foo".to_string(),
                bar: 42,
            });

        assert_eq!(expected, deserialized);
    }
}
