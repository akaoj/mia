use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::AppState;
use crate::errors::MiaError;
use crate::models::ServiceAccount;

pub async fn list_service_accounts(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<ServiceAccount>>), MiaError> {
    Ok((
        StatusCode::OK,
        Json(crate::controllers::service_accounts::list_service_accounts(state).await?),
    ))
}
pub async fn get_service_account(Path(name): Path<String>, State(state): State<AppState>) -> Result<(StatusCode, Json<ServiceAccount>), MiaError> {
    Ok((
        StatusCode::OK,
        Json(crate::controllers::service_accounts::get_service_account(state, name).await?),
    ))
}
