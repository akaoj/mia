use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};

use crate::AppState;
use crate::errors::MiaError;
use crate::models::{
    Request,
};


pub async fn list(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Request>>), MiaError> {
    Ok((
        StatusCode::OK,
        Json(crate::controllers::requests::list(state).await?),
    ))
}
