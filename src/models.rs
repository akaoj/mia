use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct ServiceAccount {
    pub name: String,
}
