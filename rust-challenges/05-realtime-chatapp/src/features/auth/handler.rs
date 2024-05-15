use axum::{http::{HeaderMap, StatusCode}, Json};

use super::model::{Claims, LoginRequest, LoginResponse};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<LoginResponse>) {
    let LoginRequest { email, password } = payload;

    let claims = Claims {
        sub: email,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let secret = "my_secret";

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    let resp = LoginResponse {
        msg: String::from("Login Successfully!"),
        token: token,
    };

    (StatusCode::CREATED, Json(resp))
}

pub async fn verify(header_map: HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(token) = header_map.get("Authorization") {
        let token = token.to_str().unwrap().replace("Bearer ", "");
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret("my_secret".as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                return Ok(Json(token_data.claims.sub));
            }
            Err(_) => {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}