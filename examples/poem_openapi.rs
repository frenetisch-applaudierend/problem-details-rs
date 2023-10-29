use http::StatusCode;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use problem_details::ProblemDetails;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(
        &self,
        name: Query<Option<String>>,
    ) -> Result<PlainText<String>, ProblemDetails> {
        match name.0 {
            Some(name) => Ok(PlainText(format!("hello, {name}!"))),
            None => Err(ProblemDetails::from_status_code(StatusCode::BAD_REQUEST)),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}
