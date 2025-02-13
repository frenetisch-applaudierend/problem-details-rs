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
//! }
//! ```
use actix_web::{
    web::Json,
    {HttpResponse, ResponseError}
};
use std::fmt::Debug;
use http::StatusCode;

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
    fn error_response(&self) -> HttpResponse {
        self.clone().into()
    }
}

#[cfg(feature = "json")]
impl<Ext> ResponseError for JsonProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn error_response(&self) -> HttpResponse {
        self.clone().into()
    }
}

#[cfg(feature = "xml")]
impl<Ext> ResponseError for XmlProblemDetails<Ext>
where
    Ext: serde::Serialize + Clone + Send + Debug,
{
    fn error_response(&self) -> HttpResponse {
        self.clone().into()
    }
}

#[cfg(feature = "json")]
impl<Ext> From<ProblemDetails<Ext>> for HttpResponse
where
    Ext: serde::Serialize + Clone + Send,

{
    fn from(value: ProblemDetails<Ext>) -> Self {
        JsonProblemDetails(value).into()
    }
}

#[cfg(feature = "json")]
impl<Ext> From<JsonProblemDetails<Ext>> for HttpResponse
where
    Ext: serde::Serialize + Clone + Send,

{
    fn from(value: JsonProblemDetails<Ext>) -> Self {
        let status_code = value.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            .as_u16();
        HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
            .content_type(JsonProblemDetails::<Ext>::CONTENT_TYPE)
            .json(Json(value.0))
    }
}

#[cfg(feature = "xml")]
impl<Ext> From<XmlProblemDetails<Ext>> for HttpResponse
where
    Ext: serde::Serialize + Clone + Send,
{
    fn from(value: XmlProblemDetails<Ext>) -> Self {
        let status_code = value.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            .as_u16();
        let content = match value.to_body_string() {
            Ok(xml) => xml,
            Err(_) => return HttpResponse::InternalServerError().into(),
        };

        HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
            .content_type(XmlProblemDetails::<Ext>::CONTENT_TYPE)
            .body(content)
    }
}

