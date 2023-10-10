//! Axum response types for [`ProblemDetails`].
//! 
//! Requires feature `axum`.
//! 
//! With the `axum` feature enabled, [`ProblemDetails`] implements [`IntoResponse`] using
//! [`JsonProblemDetails`]. You can also return [`JsonProblemDetails`] to be specific. 
//! If you want to return XML, you can use [`XmlProblemDetails`].
//! 
//! # Example
//! 
//! ```rust
//! use axum::{routing::get, Router};
//! use http::StatusCode;
//! use problem_details::ProblemDetails;
//! 
//! async fn handler() -> Result<&'static str, ProblemDetails> {
//!     // always return a problem description
//!     Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
//!         .with_detail("short and stout"))
//! }
//! 
//! fn main() {
//!     let app = Router::new().route("/", get(handler));
//!     # let _app: Router = app;
//!     // build and run server...
//! }
//! ```
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::{header, StatusCode};

use crate::ProblemDetails;

/// Response type for axum that sends a [`ProblemDetails`] as JSON.
/// 
/// Requires features `axum` and `xml`.
/// 
/// # Example
/// 
/// ```rust
/// use http::StatusCode;
/// use problem_details::{axum::JsonProblemDetails, ProblemDetails};
/// 
/// async fn handler() -> JsonProblemDetails {
///     ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
///         .with_detail("short and stout")
///         .into()
/// }
/// ```
pub struct JsonProblemDetails(ProblemDetails);

impl From<ProblemDetails> for JsonProblemDetails {
    fn from(value: ProblemDetails) -> Self {
        Self(value)
    }
}

impl IntoResponse for JsonProblemDetails {
    fn into_response(self) -> Response {
        let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (
            status_code,
            [(header::CONTENT_TYPE, "application/problem+json")],
            Json(self.0),
        )
            .into_response()
    }
}

/// Response type for axum that sends a [`ProblemDetails`] as XML.
/// 
/// Requires features `axum` and `xml`.
/// 
/// # Example
/// 
/// ```rust
/// use http::StatusCode;
/// use problem_details::{axum::XmlProblemDetails, ProblemDetails};
/// 
/// async fn handler() -> XmlProblemDetails {
///     ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
///         .with_detail("short and stout")
///         .into()
/// }
/// ```
#[cfg(feature = "xml")]
pub struct XmlProblemDetails(ProblemDetails);

#[cfg(feature = "xml")]
impl From<ProblemDetails> for XmlProblemDetails {
    fn from(value: ProblemDetails) -> Self {
        Self(value)
    }
}

#[cfg(feature = "xml")]
impl IntoResponse for XmlProblemDetails {
    fn into_response(self) -> Response {
        let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let xml = match quick_xml::se::to_string_with_root("problem", &self.0) {
            Ok(xml) => format!(r#"<?xml version="1.0" encoding="UTF-8"?>{}"#, xml),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        (
            status_code,
            [(header::CONTENT_TYPE, "application/problem+xml")],
            xml,
        )
            .into_response()
    }
}

// default is JSON
impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        JsonProblemDetails(self).into_response()
    }
}
