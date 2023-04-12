use crate::errors::MiaError;
use crate::models::{
    Request,
    ServiceAccount,
};

pub trait DB {
    fn list_requests(&self) -> Result<Vec<Request>, MiaError>;
    fn request_create_service_account(&self, service_account: ServiceAccount) -> Result<Request, MiaError>;
    fn create_service_account(&self, service_account: ServiceAccount) -> Result<ServiceAccount, MiaError>;
    fn list_service_accounts(&self) -> Result<Vec<ServiceAccount>, MiaError>;
    fn get_service_account(&self, name: &str) -> Result<ServiceAccount, MiaError>;
}
