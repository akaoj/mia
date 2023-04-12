use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::AppState;
use crate::errors::MiaError;
use crate::models::{
    Request,
    ServiceAccount,
};


pub async fn create(State(state): State<AppState>, Json(service_account): Json<ServiceAccount>) -> Result<(StatusCode, Json<Request>), MiaError> {
    Ok((
        StatusCode::CREATED,
        Json(crate::controllers::service_accounts::create(state, service_account).await?),
    ))
}

pub async fn list(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<ServiceAccount>>), MiaError> {
    Ok((
        StatusCode::OK,
        Json(crate::controllers::service_accounts::list(state).await?),
    ))
}

pub async fn get(Path(name): Path<String>, State(state): State<AppState>) -> Result<(StatusCode, Json<ServiceAccount>), MiaError> {
    Ok((
        StatusCode::OK,
        Json(crate::controllers::service_accounts::get(state, name).await?),
    ))
}
