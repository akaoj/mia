use crate::AppState;
use crate::errors::MiaError;
use crate::models::ServiceAccount;

pub async fn list_service_accounts(state: AppState) -> Result<Vec<ServiceAccount>, MiaError> {
    state.db.lock().await.list_service_accounts()
}

pub async fn get_service_account(state: AppState, name: String) -> Result<ServiceAccount, MiaError> {
    state.db.lock().await.get_service_account(&name)
}
