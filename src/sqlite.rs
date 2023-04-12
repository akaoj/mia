use std::path::PathBuf;

use chrono::{Utc};

use crate::errors::MiaError;
use crate::models::{
    Request,
    RequestKind,
    RequestStatus,
    ServiceAccount,
};
use crate::utils::DB;


pub struct SQLiteDB {
    connection: rusqlite::Connection,
}

impl SQLiteDB {
    pub fn new(path: PathBuf) -> Result<Self, rusqlite::Error> {
        let connection = rusqlite::Connection::open(path)?;
        Ok(SQLiteDB{ connection: connection })
    }
}

impl DB for SQLiteDB {
    fn list_requests(&self) -> Result<Vec<Request>, MiaError> {
        Ok(vec![
            Request {
                id: "83938".to_owned(),
                creation_date: Utc::now(),
                kind: RequestKind::ServiceAccountCreate {
                    name: "toto".to_owned(),
                },
                statuses: vec![RequestStatus::WaitingForApproval(Utc::now())],
                expiration_date: None,
            },
            Request {
                id: "0863".to_owned(),
                creation_date: Utc::now(),
                kind: RequestKind::ServiceAccountDelete {
                    id: "93982".to_owned(),
                },
                statuses: vec![
                    RequestStatus::WaitingForApproval(Utc::now()),
                    RequestStatus::ApprovedWaitingForProcessing(Utc::now()),
                    RequestStatus::ProcessedWaitingForExpiration(Utc::now()),
                    RequestStatus::Done(Utc::now()),
                ],
                expiration_date: None,
            },
        ])
    }

    fn request_create_service_account(&self, service_account: ServiceAccount) -> Result<Request, MiaError> {
        Ok(Request {
            id: "0833".to_owned(),
            creation_date: Utc::now(),
            kind: RequestKind::ServiceAccountCreate {
                name: service_account.name,
            },
            statuses: vec![
                RequestStatus::WaitingForApproval(Utc::now()),
                RequestStatus::ApprovedWaitingForProcessing(Utc::now()),
            ],
            expiration_date: None,
        })
    }

    fn create_service_account(&self, service_account: ServiceAccount) -> Result<ServiceAccount, MiaError> {
        Ok(ServiceAccount{
            id: "8698".to_owned(),
            name: "toto".to_owned(),
        })
    }

    fn list_service_accounts(&self) -> Result<Vec<ServiceAccount>, MiaError> {
        Ok(vec![
            ServiceAccount{
                id: "8698".to_owned(),
                name: String::from("mysa"),
            },
            ServiceAccount{
                id: "828698".to_owned(),
                name: String::from("anothersa"),
            },
        ])
    }

    fn get_service_account(&self, name: &str) -> Result<ServiceAccount, MiaError> {
        Ok(ServiceAccount{
            id: "9738698".to_owned(),
            name: name.to_owned(),
        })
    }
}
