use crate::AppState;
use crate::errors::MiaError;
use crate::models::{
    Request,
    ServiceAccount,
};

pub async fn create(state: AppState, service_account: ServiceAccount) -> Result<Request, MiaError> {
    state.db.lock().await.request_create_service_account(service_account)
}

pub async fn get(state: AppState, name: String) -> Result<ServiceAccount, MiaError> {
    state.db.lock().await.get_service_account(&name)
}

pub async fn list(state: AppState) -> Result<Vec<ServiceAccount>, MiaError> {
    state.db.lock().await.list_service_accounts()
}
