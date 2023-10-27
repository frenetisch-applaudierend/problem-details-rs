use http::StatusCode;
use poem::{get, handler, listener::TcpListener, Route, Server};
use problem_details::{JsonProblemDetails, ProblemDetails, XmlProblemDetails};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/", get(default))
        .at("/json", get(json))
        .at("/xml", get(xml));

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

#[handler]
async fn default() -> Result<&'static str, ProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT).with_detail("short and stout"))
}

#[handler]
async fn json() -> Result<&'static str, JsonProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
        .with_detail("short and stout")
        .into())
}

#[handler]
async fn xml() -> Result<&'static str, XmlProblemDetails> {
    // always return an error with a problem description
    // NOTE: some browsers don't like the content type application/problem+xml and report an error
    //       like "invalid content" or similar. Use curl instead to see the response in this case.
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT)
        .with_detail("short and stout")
        .into())
}
