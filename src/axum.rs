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
    Json,
    response::{IntoResponse, Response},
};
use http::{StatusCode, header};

use crate::ProblemDetails;

#[cfg(feature = "json")]
use crate::JsonProblemDetails;

#[cfg(feature = "xml")]
use crate::XmlProblemDetails;

#[cfg(feature = "json")]
impl<Ext> IntoResponse for JsonProblemDetails<Ext>
where
    Ext: serde::Serialize,
{
    fn into_response(self) -> Response {
        let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let content_type = [(header::CONTENT_TYPE, Self::CONTENT_TYPE)];
        let content = Json(self.0);

        (status_code, content_type, content).into_response()
    }
}

#[cfg(feature = "xml")]
impl<Ext> IntoResponse for XmlProblemDetails<Ext>
where
    Ext: serde::Serialize,
{
    fn into_response(self) -> Response {
        let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let content_type = [(header::CONTENT_TYPE, Self::CONTENT_TYPE)];
        let content = match self.to_body_string() {
            Ok(xml) => xml,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        (status_code, content_type, content).into_response()
    }
}

#[cfg(feature = "json")]
impl<Ext> IntoResponse for ProblemDetails<Ext>
where
    Ext: serde::Serialize,
{
    fn into_response(self) -> Response {
        JsonProblemDetails(self).into_response()
    }
}
