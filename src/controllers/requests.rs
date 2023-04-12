use crate::AppState;
use crate::errors::MiaError;
use crate::models::{
    Request,
};


pub async fn list(state: AppState) -> Result<Vec<Request>, MiaError> {
    state.db.lock().await.list_requests()
}
