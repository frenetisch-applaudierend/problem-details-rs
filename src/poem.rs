//! Poem response types for [`ProblemDetails`]. Requires feature `poem`.
//!
//! With the `poem` feature enabled, [`ProblemDetails`] implements [`IntoResponse`] using
//! [`JsonProblemDetails`]. You can also return [`JsonProblemDetails`] to be specific.
//! If you want to return XML, you can use [`XmlProblemDetails`] (requires feature `xml`).
//!
//! # Example
//!
//! ```rust
//! use poem::{get, Route};
//! use http::StatusCode;
//! use problem_details::ProblemDetails;
//!
//! #[poem::handler]
//! async fn handler() -> Result<&'static str, ProblemDetails> {
//!     // always return a problem description
//!     Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
//!         .with_detail("short and stout"))
//! }
//!
//! fn main() {
//!     let app = Route::new().at("/", get(handler));
//!     # let _app = app;
//!     // build and run server...
//! }
//! ```
//!
//! # OpenAPI
//!
//! With the `poem-openapi` feature enabled (which implies `poem` and `json`),
//! [`ProblemDetails`] additionally implements [`poem_openapi::ApiResponse`], so it can be
//! used as the error type of an `#[OpenApi]` handler. Its schema is registered once as
//! `ProblemDetails` and reused across handlers.
//!
//! ```rust
//! # #[cfg(feature = "poem-openapi")]
//! # mod example {
//! use http::StatusCode;
//! use poem_openapi::{param::Query, payload::PlainText, OpenApi};
//! use problem_details::ProblemDetails;
//!
//! struct Api;
//!
//! #[OpenApi]
//! impl Api {
//!     #[oai(path = "/hello", method = "get")]
//!     async fn index(
//!         &self,
//!         name: Query<Option<String>>,
//!     ) -> Result<PlainText<String>, ProblemDetails> {
//!         match name.0 {
//!             Some(name) => Ok(PlainText(format!("hello, {name}!"))),
//!             None => Err(ProblemDetails::from_status_code(StatusCode::BAD_REQUEST)),
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! If your [`ProblemDetails`] uses a custom extensions type, implement [`Extension`] for it
//! so its fields are merged into the generated schema.
use http::StatusCode;
use poem::{IntoResponse, Response, error::ResponseError, web::Json};

use crate::ProblemDetails;

#[cfg(feature = "json")]
use crate::JsonProblemDetails;

#[cfg(feature = "xml")]
use crate::XmlProblemDetails;

impl<Ext> ResponseError for ProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send,
{
    fn status(&self) -> StatusCode {
        self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn as_response(&self) -> poem::Response {
        self.clone().into_response()
    }
}

#[cfg(feature = "json")]
impl<Ext> ResponseError for JsonProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send,
{
    fn status(&self) -> StatusCode {
        self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn as_response(&self) -> poem::Response {
        self.clone().into_response()
    }
}

#[cfg(feature = "xml")]
impl<Ext> ResponseError for XmlProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send,
{
    fn status(&self) -> StatusCode {
        self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn as_response(&self) -> poem::Response {
        self.clone().into_response()
    }
}

#[cfg(feature = "json")]
impl<Ext> IntoResponse for JsonProblemDetails<Ext>
where
    Ext: serde::Serialize + Send,
{
    fn into_response(self) -> Response {
        let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let content = Json(self.0).with_content_type(Self::CONTENT_TYPE);

        (status_code, content).into_response()
    }
}

#[cfg(feature = "xml")]
impl<Ext> IntoResponse for XmlProblemDetails<Ext>
where
    Ext: serde::Serialize + Send,
{
    fn into_response(self) -> Response {
        let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let content = match self.to_body_string() {
            Ok(xml) => xml,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };
        let content = content.with_content_type(Self::CONTENT_TYPE);

        (status_code, content).into_response()
    }
}

#[cfg(feature = "json")]
impl<Ext> IntoResponse for ProblemDetails<Ext>
where
    Ext: serde::Serialize + Send,
{
    fn into_response(self) -> Response {
        JsonProblemDetails(self).into_response()
    }
}

/// Provides the OpenAPI schema for the `extensions` field of [`ProblemDetails`] when using
/// the `poem-openapi` feature.
///
/// The extensions are flattened into the top-level problem details object (see
/// [`ProblemDetails::extensions`]), so their schema needs to be merged into
/// [`ProblemDetails`]'s own schema instead of being nested under a property. Implement this
/// trait for your own extension types to use them with `poem-openapi`; it is already
/// implemented for `()` (no extensions) and for
/// [`HashMap<String, serde_json::Value>`](std::collections::HashMap) (dynamic extensions, see
/// [`ProblemDetails`] for an example).
#[cfg(feature = "poem-openapi")]
pub trait Extension {
    /// A short, unique name identifying this extension type.
    ///
    /// This is used to build a unique OpenAPI schema name for `ProblemDetails<Self>`, since
    /// different extension types need distinct schemas in the registry.
    fn schema_suffix() -> std::borrow::Cow<'static, str>;

    /// Registers this extension's schema in the registry.
    ///
    /// Returns a [`MetaSchema`](poem_openapi::registry::MetaSchema) whose `properties`,
    /// `required` and `additional_properties` are merged into `ProblemDetails`'s own schema.
    fn register(
        registry: &mut poem_openapi::registry::Registry,
    ) -> poem_openapi::registry::MetaSchema;
}

#[cfg(feature = "poem-openapi")]
impl Extension for () {
    fn schema_suffix() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("")
    }

    fn register(
        _registry: &mut poem_openapi::registry::Registry,
    ) -> poem_openapi::registry::MetaSchema {
        poem_openapi::registry::MetaSchema::new("object")
    }
}

#[cfg(feature = "poem-openapi")]
impl Extension for std::collections::HashMap<String, serde_json::Value> {
    fn schema_suffix() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("map_any")
    }

    fn register(
        _registry: &mut poem_openapi::registry::Registry,
    ) -> poem_openapi::registry::MetaSchema {
        use poem_openapi::registry::{MetaSchema, MetaSchemaRef};

        MetaSchema {
            additional_properties: Some(Box::new(MetaSchemaRef::Inline(Box::new(MetaSchema::ANY)))),
            ..MetaSchema::new("object")
        }
    }
}

#[cfg(feature = "poem-openapi")]
impl<Ext> poem_openapi::ApiResponse for ProblemDetails<Ext>
where
    Ext: Extension + Send + 'static,
{
    fn meta() -> poem_openapi::registry::MetaResponses {
        use poem_openapi::registry::{MetaMediaType, MetaResponse, MetaResponses, MetaSchemaRef};

        MetaResponses {
            responses: vec![MetaResponse {
                description: "An RFC 9457 / RFC 7807 problem details object",
                status: None,
                status_range: None,
                content: vec![MetaMediaType {
                    content_type: "application/problem+json",
                    schema: MetaSchemaRef::Reference(Self::schema_name()),
                }],
                headers: vec![],
            }],
        }
    }

    fn register(registry: &mut poem_openapi::registry::Registry) {
        use poem_openapi::registry::{MetaSchema, MetaSchemaRef};

        registry.create_schema::<Self, _>(Self::schema_name(), |registry| {
            let ext_schema = Ext::register(registry);

            let mut properties = vec![
                (
                    "type",
                    MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "uri"))),
                ),
                (
                    "status",
                    MetaSchemaRef::Inline(Box::new(MetaSchema::new("integer"))),
                ),
                (
                    "title",
                    MetaSchemaRef::Inline(Box::new(MetaSchema::new("string"))),
                ),
                (
                    "detail",
                    MetaSchemaRef::Inline(Box::new(MetaSchema::new("string"))),
                ),
                (
                    "instance",
                    MetaSchemaRef::Inline(Box::new(MetaSchema::new_with_format("string", "uri"))),
                ),
            ];
            properties.extend(ext_schema.properties);

            MetaSchema {
                description: Some("RFC 9457 / RFC 7807 problem details"),
                properties,
                required: ext_schema.required,
                additional_properties: ext_schema.additional_properties,
                ..MetaSchema::new("object")
            }
        });
    }
}

#[cfg(feature = "poem-openapi")]
impl<Ext> ProblemDetails<Ext>
where
    Ext: Extension,
{
    fn schema_name() -> String {
        let suffix = Ext::schema_suffix();
        if suffix.is_empty() {
            "ProblemDetails".to_string()
        } else {
            format!("ProblemDetails_{suffix}")
        }
    }
}

#[cfg(all(test, feature = "poem-openapi"))]
mod poem_openapi_tests {
    use std::collections::HashMap;

    use poem_openapi::registry::{MetaSchemaRef, Registry};

    use super::*;

    #[test]
    fn registers_schema_for_default_extensions() {
        let mut registry = Registry::new();
        <ProblemDetails as poem_openapi::ApiResponse>::register(&mut registry);

        let schema = registry
            .schemas
            .get("ProblemDetails")
            .expect("schema should be registered");

        let property_names: Vec<_> = schema.properties.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            property_names,
            vec!["type", "status", "title", "detail", "instance"]
        );
        assert!(schema.required.is_empty());
        assert!(schema.additional_properties.is_none());
    }

    #[test]
    fn registers_distinct_schema_for_map_extensions() {
        let mut registry = Registry::new();
        <ProblemDetails<HashMap<String, serde_json::Value>> as poem_openapi::ApiResponse>::register(
            &mut registry,
        );

        let schema = registry
            .schemas
            .get("ProblemDetails_map_any")
            .expect("schema should be registered under a distinct name");

        assert!(matches!(
            schema.additional_properties.as_deref(),
            Some(MetaSchemaRef::Inline(_))
        ));
    }

    #[test]
    fn registering_the_same_schema_twice_does_not_panic() {
        let mut registry = Registry::new();
        <ProblemDetails as poem_openapi::ApiResponse>::register(&mut registry);
        <ProblemDetails as poem_openapi::ApiResponse>::register(&mut registry);
    }
}
