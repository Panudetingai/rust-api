use axum::{
    extract::Request,
    http::{StatusCode, header::CONTENT_TYPE},
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Log IP without consuming req
    log_ip(&req);

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let body = "{\"error\": \"Unauthorized: Invalid token\", \"status\": 401}";
    let response = Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(CONTENT_TYPE, "application/json")
        .body(body.into())
        .unwrap();

    if let Some(auth_header) = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ").map(str::trim) {
            if !token.is_empty() {
                let response = next.run(req).await;
                return Ok(response);
            }
        }
        Ok(response)
    } else {
        Ok(response)
    }
}
fn log_ip(req: &Request) {
    if let Some(ip) = req.extensions().get::<std::net::SocketAddr>() {
        tracing::info!("Incoming request from IP: {}", ip.ip());
    }
}
