use std::path::PathBuf;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    routing::{
        get,
        post,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Router,
};
use serde_json::json;
use tokio::sync::Mutex;

mod controllers;
mod errors;
mod models;
mod sqlite;
mod utils;
mod views;
use crate::errors::MiaError;
use crate::utils::DB;
use crate::views::requests;
use crate::views::service_accounts;

#[derive(Clone)]
pub struct AppState {
    // XXX: change the mutex to a connection pool?
    db: Arc<Mutex<dyn DB + Send>>,
}

#[tokio::main]
async fn main() {
    let db = crate::sqlite::SQLiteDB::new(PathBuf::from("./db.sqlite")).unwrap();

    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let app = Router::new()
        .route("/requests/", get(requests::list))
        .route("/service-accounts/", get(service_accounts::list))
        .route("/service-accounts/", post(service_accounts::create))
        .route("/service-accounts/:name", get(service_accounts::get))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

impl IntoResponse for errors::MiaError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            MiaError::ServiceAccountNotFound(svc) => (StatusCode::NOT_FOUND, format!("Service account {} no found", svc)),
            MiaError::UnknownError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err)),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
