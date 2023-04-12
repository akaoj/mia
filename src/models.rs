use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct ServiceAccount {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Human {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "id", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OwnerKind {
    ServiceAccount(String),
    Group(String),
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "id", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MemberKind {
    Human(String),
    ServiceAccount(String),
    Group(String),
}

// Example:
//   id=42  domain=mssql     name=idm_db                      scope=
//   id=28  domain=vault     name=secrets/myapp/creds         scope=
//   id=29  domain=vault     name=secrets/myapp/creds/passwd  scope=
//   id=91  domain=kube      name=idm-api                     scope=idm
//   id=10  domain=gerrit    name=idm-api                     scope=identity-management
#[derive(Deserialize, Serialize)]
pub struct Resource {
    pub id: String,
    pub owners: Vec<OwnerKind>,
    pub domain: String,
    pub name: String,
    pub scope: Option<String>,
    pub env: Option<String>,
}

// Example:
//   resource=42  name=ro    description="Read-only access to the database"
//   resource=10  name=push  description="Allow push to the repo"
#[derive(Deserialize, Serialize)]
pub struct ResourceGrant {
    pub id: String,
    pub resource_id: String,
    pub members: Vec<MemberKind>,
    pub name: String,
    pub description: Option<String>,
}


#[derive(Deserialize, Serialize)]
pub struct Request {
    pub id: String,
    pub creation_date: DateTime<Utc>,
    pub kind: RequestKind,
    pub statuses: Vec<RequestStatus>,
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestKind {
    ServiceAccountCreate { name: String },
    ServiceAccountDelete { id: String },
    ServiceAccountOwnershipAdd { service_account_id: String, owner: OwnerKind },
    GroupMembershipAdd { group_id: String, member: MemberKind },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", content = "date", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestStatus {
    WaitingForApproval(DateTime<Utc>),
    ApprovedWaitingForProcessing(DateTime<Utc>),
    ProcessedWaitingForExpiration(DateTime<Utc>),
    Done(DateTime<Utc>),
    Denied(DateTime<Utc>),
}
