use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    #[tokio::test]
    async fn test_healthcheck() {
        let response = healthcheck().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"OK")
    }
}
