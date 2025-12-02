use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::auth::JwtService;
use crate::models::user::Claims;
use std::sync::Arc;

pub async fn auth_middleware(
    State(jwt_service): State<JwtService>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        });

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = jwt_service.validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add user claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// Extension trait for extracting user from request
pub trait UserExt {
    fn user_id(&self) -> Option<String>;
    fn username(&self) -> Option<String>;
}

impl<T> UserExt for Request<T> {
    fn user_id(&self) -> Option<String> {
        self.extensions()
            .get::<Claims>()
            .map(|claims| claims.sub.clone())
    }

    fn username(&self) -> Option<String> {
        self.extensions()
            .get::<Claims>()
            .map(|claims| claims.username.clone())
    }
}
