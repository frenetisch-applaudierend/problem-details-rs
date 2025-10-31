use http::{StatusCode, Uri};

use crate::ProblemType;

#[cfg(feature = "json")]
mod json;

#[cfg(feature = "json")]
pub use json::JsonProblemDetails;

#[cfg(feature = "xml")]
mod xml;

#[cfg(feature = "xml")]
pub use xml::XmlProblemDetails;

#[cfg(test)]
mod tests;

/// A RFC 9457 / RFC 7807 problem details object.
///
/// # Creating problem details
///
/// You can create a new problem details from a given
/// status code using [`ProblemDetails::from_status_code`].
///
/// This will set the [`status`](ProblemDetails::status) field to the given status code,
/// the [`title`](ProblemDetails::title) field to the canonical reason phrase of the status code,
/// and the [`type`](ProblemDetails::type) field to none, which is equivalent to `about:blank`.
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
/// # Extensions
///
/// To add extensions, you need to define a struct that holds the extension
/// fields, and use this struct as the generic parameter for [`ProblemDetails<Ext>`].
/// Using [`with_extensions`](ProblemDetails::with_extensions), the type is adjusted
/// automatically for you.
///
/// Extension fields are flattened into the problem details object when serialized.
///
/// ```rust
/// use problem_details::ProblemDetails;
///
/// struct MyExt {
///     foo: String,
///     bar: u32,
/// }
///
/// let details = ProblemDetails::new()
///     .with_extensions(MyExt {
///         foo: "Hello".to_string(),
///         bar: 42,
///     });
///
/// // details is of type ProblemDetails<MyExt>
/// let typecheck: ProblemDetails<MyExt> = details;
/// ```
///
/// If you need dynamic extensions, you can use a [`HashMap`](std::collections::HashMap)
/// as extensions object.
///
/// ```rust
/// use std::collections::HashMap;
/// use problem_details::ProblemDetails;
///
/// let mut extensions = HashMap::<String, serde_json::Value>::new();
/// extensions.insert("foo".to_string(), serde_json::json!("Hello"));
/// extensions.insert("bar".to_string(), serde_json::json!(42));
///
/// let details = ProblemDetails::new()
///    .with_extensions(extensions);
///
/// // details is of type ProblemDetails<HashMap<String, serde_json::Value>>
/// let typecheck: ProblemDetails<HashMap<String, serde_json::Value>> = details;
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(
    feature = "utoipa",
    schema(description = "RFC 9457 / RFC 7807 problem details")
)]
pub struct ProblemDetails<Ext = ()> {
    /// An optional uri describing the problem type.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-type]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = Uri))]
    pub r#type: Option<ProblemType>,

    /// An optional status code for this problem.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-status]() for more information.
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::status::opt"))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "utoipa", schema(value_type = u16))]
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
    #[cfg_attr(feature = "utoipa", schema(value_type = String, format = Uri))]
    pub instance: Option<Uri>,

    /// An object containing extensions to this problem details object.
    ///
    /// Note that the extensions will be flattened into the resulting problem details
    /// representation.
    ///
    /// See [https://www.rfc-editor.org/rfc/rfc9457.html#name-extension-members]() for more information.
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[schema(inline)]
    pub extensions: Ext,
}

impl ProblemDetails<()> {
    /// Creates a new empty problem details object.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn with_type(mut self, r#type: impl Into<ProblemType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Builder-style method that sets the `status` field of this problem details object.
    #[must_use]
    pub fn with_status(mut self, status: impl Into<StatusCode>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Builder-style method that sets the `title` field of this problem details object.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Builder-style method that sets the `detail` field of this problem details object.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Builder-style method that sets the `instance` field of this problem details object.
    #[must_use]
    pub fn with_instance(mut self, instance: impl Into<Uri>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// Builder style method that sets the `extensions` field of this probelm details object.
    #[must_use]
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
