use axum::{routing::get, Router};
use http::StatusCode;
use problem_details::ProblemDetails;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Result<&'static str, ProblemDetails> {
    // always return a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT).with_detail("short and stout"))
}
