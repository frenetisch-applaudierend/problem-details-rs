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
