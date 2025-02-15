use actix_web::{web, App, HttpServer};
use http::StatusCode;
use problem_details::{JsonProblemDetails, ProblemDetails, XmlProblemDetails};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(default))
            .route("/json", web::get().to(json))
            .route("/xml", web::get().to(xml))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
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
