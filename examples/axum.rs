use axum::{routing::get, Router};
use http::StatusCode;
use problem_details::{JsonProblemDetails, ProblemDetails, XmlProblemDetails};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(default))
        .route("/json", get(json))
        .route("/xml", get(xml));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn default() -> Result<&'static str, ProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT).with_detail("short and stout"))
}

async fn json() -> Result<&'static str, JsonProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
        .with_detail("short and stout")
        .into())
}

async fn xml() -> Result<&'static str, XmlProblemDetails> {
    // always return an error with a problem description
    // NOTE: some browsers don't like the content type application/problem+xml and report an error
    //       like "invalid content" or similar. Use curl instead to see the response in this case.
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
        .with_detail("short and stout")
        .into())
}
