use axum::{routing::post, Router};

use super::handler::{login, verify};

pub fn get_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/verify", post(verify))
}
