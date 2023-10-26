use http::StatusCode;
use poem::{get, handler, listener::TcpListener, Route, Server};
use problem_details::ProblemDetails;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}

#[handler]
async fn hello() -> Result<&'static str, ProblemDetails> {
    // always return an error with a problem description
    Err(ProblemDetails::from_status_code(StatusCode::IM_A_TEAPOT).with_detail("short and stout"))
}
