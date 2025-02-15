//! Actix response types for [`ProblemDetails`].
//!
//! Requires feature `actix`.
//!
//! With the `actix` feature enabled, [`ProblemDetails`] implements [`ResponseError`] using
//! [`JsonProblemDetails`]. You can also return [`JsonProblemDetails`] to be specific.
//! If you want to return XML, you can use [`XmlProblemDetails`].
//!
//! # Example
//!
//! ```rust
//! use actix_web::{App, web, HttpServer};
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
//!     HttpServer::new(|| {
//!         App::new()
//!             .route("/", web::get().to(handler))
//!     // build and run server...
//!     });
//! }
//! ```
use actix_web::{
    web::Json,
    {HttpResponse, ResponseError},
};
use http::StatusCode;
use std::fmt::Debug;

use crate::ProblemDetails;

#[cfg(feature = "json")]
use crate::JsonProblemDetails;

#[cfg(feature = "xml")]
use crate::XmlProblemDetails;

#[cfg(feature = "json")]
impl<Ext> ResponseError for ProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        // Due to http crate version mismatches we need to translate the status code
        let status_code = self
            .status
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            .as_u16();
        actix_web::http::StatusCode::from_u16(status_code)
            .expect("Status code should be translatable")
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(JsonProblemDetails::<Ext>::CONTENT_TYPE)
            .json(Json(self))
    }
}

#[cfg(feature = "json")]
impl<Ext> ResponseError for JsonProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.0.status_code())
            .content_type(JsonProblemDetails::<Ext>::CONTENT_TYPE)
            .json(Json(self.0.clone()))
    }
}

#[cfg(feature = "xml")]
impl<Ext> ResponseError for XmlProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn error_response(&self) -> HttpResponse {
        let content = match self.to_body_string() {
            Ok(xml) => xml,
            Err(_) => return HttpResponse::InternalServerError().into(),
        };

        HttpResponse::build(self.0.status_code())
            .content_type(XmlProblemDetails::<Ext>::CONTENT_TYPE)
            .body(content)
    }
}
