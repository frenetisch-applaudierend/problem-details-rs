use axum::{Json, routing::get};
use http::StatusCode;
use problem_details::{JsonProblemDetails, ProblemDetails, XmlProblemDetails};
use tokio::net::TcpListener;
use utoipa_axum::{router::OpenApiRouter, routes};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let (app, api) = OpenApiRouter::new()
        .routes(routes!(default))
        .routes(routes!(json))
        .routes(routes!(xml))
        .split_for_parts();

    let app = app.route("/openapi.json", get(move || async move { Json(api) }));

    // run it with hyper on localhost:3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// ProblemDetails in default format
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = String, description = "Success"),
        (status = IM_A_TEAPOT, body = ProblemDetails, description = "I'm a teapot!"),
    ),
)]
async fn default() -> Result<&'static str, ProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT).with_detail("short and stout"))
}

/// ProblemDetails in JSON format
#[utoipa::path(
    get,
    path = "/json",
    responses(
        (status = OK, body = String, description = "Success"),
        (
            status = IM_A_TEAPOT,
            body = ProblemDetails,
            description = "I'm a teapot!",
            content_type = JsonProblemDetails::<()>::CONTENT_TYPE,
        ),
    ),
)]
async fn json() -> Result<&'static str, JsonProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
        .with_detail("short and stout")
        .into())
}

/// ProblemDetails in XML format
#[utoipa::path(
    get,
    path = "/xml",
    responses(
        (status = OK, body = String, description = "Success"),
        (
            status = IM_A_TEAPOT,
            body = ProblemDetails,
            description = "I'm a teapot!",
            content_type = XmlProblemDetails::<()>::CONTENT_TYPE,
        ),
    ),
)]
async fn xml() -> Result<&'static str, XmlProblemDetails> {
    // always return an error with a problem description
    // NOTE: some browsers don't like the content type application/problem+xml and report an error
    //       like "invalid content" or similar. Use curl instead to see the response in this case.
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
        .with_detail("short and stout")
        .into())
}
