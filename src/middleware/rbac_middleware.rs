use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use crate::model::User;
use crate::util::AppError;

pub async fn admin_only_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let user = req.extensions().get::<User>()
        .ok_or(AppError::Unauthorized("Authentication required".to_string()))?;

    if user.role != "admin" {
        return Err(AppError::Unauthorized("You do not have permission to access this resource".to_string()));
    }

    Ok(next.run(req).await)
}
