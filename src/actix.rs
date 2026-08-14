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
#[cfg(any(feature = "json", feature = "xml"))]
use actix_web::{HttpResponse, ResponseError};
#[cfg(any(feature = "json", feature = "xml"))]
use http::StatusCode;
#[cfg(any(feature = "json", feature = "xml"))]
use std::fmt::Debug;

#[cfg(feature = "json")]
use crate::{JsonProblemDetails, ProblemDetails};

#[cfg(feature = "xml")]
use crate::XmlProblemDetails;

#[cfg(feature = "json")]
impl<Ext> ResponseError for ProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_status_code(self.status)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(JsonProblemDetails::<Ext>::CONTENT_TYPE)
            .json(self)
    }
}

#[cfg(feature = "json")]
impl<Ext> ResponseError for JsonProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_status_code(self.0.status)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type(JsonProblemDetails::<Ext>::CONTENT_TYPE)
            .json(&self.0)
    }
}

#[cfg(feature = "xml")]
impl<Ext> ResponseError for XmlProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_status_code(self.0.status)
    }

    fn error_response(&self) -> HttpResponse {
        let content = match self.to_body_string() {
            Ok(xml) => xml,
            Err(_) => return HttpResponse::InternalServerError().into(),
        };

        HttpResponse::build(self.status_code())
            .content_type(XmlProblemDetails::<Ext>::CONTENT_TYPE)
            .body(content)
    }
}

/// Due to http crate version mismatches we need to translate the status code.
#[cfg(any(feature = "json", feature = "xml"))]
fn actix_status_code(status: Option<StatusCode>) -> actix_web::http::StatusCode {
    let status_code = status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR).as_u16();
    actix_web::http::StatusCode::from_u16(status_code).expect("Status code should be translatable")
}
