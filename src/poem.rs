//! Poem response types for [`ProblemDetails`].
//!
//! Requires feature `poem`.
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
use http::{header, StatusCode};
use poem::{error::ResponseError, web::Json, IntoResponse, Response};

#[cfg(feature = "xml")]
use poem::web::Xml;

use crate::ProblemDetails;

/// Response type for poem that sends a [`ProblemDetails`] as JSON.
///
/// Requires features `poem` and `serde`.
///
/// # Example
///
/// ```rust
/// use http::StatusCode;
/// use problem_details::{poem::JsonProblemDetails, ProblemDetails};
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
        let content = Json(self.0).with_header(header::CONTENT_TYPE, "application/problem+json");

        (status_code, content).into_response()
    }
}

/// Response type for poem that sends a [`ProblemDetails`] as XML.
///
/// Requires features `poem` and `xml`.
///
/// # Example
///
/// ```rust
/// use http::StatusCode;
/// use problem_details::{poem::XmlProblemDetails, ProblemDetails};
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
        let content = Xml(self.0).with_header(header::CONTENT_TYPE, "application/problem+xml");

        (status_code, content).into_response()
    }
}

// default is JSON
impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        JsonProblemDetails(self).into_response()
    }
}

impl ResponseError for ProblemDetails {
    fn status(&self) -> StatusCode {
        self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn as_response(&self) -> poem::Response {
        self.clone().into_response()
    }
}
