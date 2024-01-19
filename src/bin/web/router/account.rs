use axum::{Json, Router};
use axum::routing::post;
use community_v2::service::response::ApplicationResponse;
use community_v2::domain::account::command::SignInAccount;
use crate::router::response::{AxumError, AxumResponse};

#[axum::debug_handler]
pub async fn sign_in_account(
    Json(payload): Json<SignInAccount>
) -> Result<AxumResponse<ApplicationResponse>, AxumError> {
    tracing::info!("sign_in_account: {:?}", payload.email);
    Ok(AxumResponse(
        ApplicationResponse::String("Hello world".to_string())
    ))
}

pub fn account_router() -> Router {
    Router::new()
        .route("/sign-in", post(sign_in_account))
}