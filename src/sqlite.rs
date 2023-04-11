use std::path::PathBuf;

use crate::errors::MiaError;
use crate::models::ServiceAccount;
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
    fn list_service_accounts(&self) -> Result<Vec<ServiceAccount>, MiaError> {
        Ok(vec![
            ServiceAccount{
                name: String::from("mysa"),
            },
            ServiceAccount{
                name: String::from("anothersa"),
            },
        ])
    }
    fn get_service_account(&self, name: &str) -> Result<ServiceAccount, MiaError> {
        Ok(ServiceAccount{
            name: name.to_string(),
        })
    }
}
