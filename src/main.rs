use lambda_http::{
    run,
    service_fn,
    Body,
    Error,
    Request,
    RequestExt,
    Response,
};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let ip = match event.request_context() {
        lambda_http::request::RequestContext::ApiGatewayV2(c) => c.http.source_ip,
    };
    return Ok(
        Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(ip.unwrap_or("".to_string()).into())
            .map_err(Box::new)?,
    );
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).with_target(false).without_time().init();
    run(service_fn(function_handler)).await
}
