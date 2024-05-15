use axum::{routing::post, Router};

use super::handler::create_user;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", post(create_user))
}
