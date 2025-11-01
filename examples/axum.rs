use axum::{Router, routing::get};
use http::StatusCode;
use problem_details::{JsonProblemDetails, ProblemDetails, XmlProblemDetails};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(default))
        .route("/json", get(json))
        .route("/xml", get(xml));

    // run it with hyper on localhost:3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
