use crate::errors::MiaError;
use crate::models::ServiceAccount;

pub trait DB {
    fn list_service_accounts(&self) -> Result<Vec<ServiceAccount>, MiaError>;
    fn get_service_account(&self, name: &str) -> Result<ServiceAccount, MiaError>;
}
