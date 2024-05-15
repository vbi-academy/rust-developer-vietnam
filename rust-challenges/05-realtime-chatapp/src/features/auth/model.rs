use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginResponse {
	pub msg: String,
	pub token: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
	pub email: String,
	pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
	pub sub: String, // the subject of the token
	pub exp: usize, // the expiry time
}