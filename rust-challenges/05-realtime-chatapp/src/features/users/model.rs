use serde::{Deserialize, Serialize};
use uuid::Uuid;

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}