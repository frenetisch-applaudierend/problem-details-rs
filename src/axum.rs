use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::{header, StatusCode};

use crate::ProblemDetails;

pub struct JsonProblemDetails(ProblemDetails);

// pub struct XmlProblemDetails(ProblemDetails);

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

// impl From<ProblemDetails> for XmlProblemDetails {
//     fn from(value: ProblemDetails) -> Self {
//         Self(value)
//     }
// }

// impl IntoResponse for XmlProblemDetails {
//     fn into_response(self) -> Response {
//         let status_code = self.0.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
//         let xml =
//         (status_code, [(header::CONTENT_TYPE, "application/problem+xml")], self.0).into_response()
//     }
// }

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        // default is JSON
        JsonProblemDetails(self).into_response()
    }
}
