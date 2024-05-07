

use actix_web::{post, web::Json};
use argon2::{password_hash::SaltString, Argon2};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{api::ApiError, db};

#[derive(ToSchema, Debug, Serialize, Deserialize, Clone)]
pub struct UserCreationRequest {
    name: String,
    #[schema(format = "email", pattern = "\\.[Ii][Oo]$")]
    email: String,
    #[schema(
        format = "password",
        pattern = r#"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]+$"#,
        min_length = 4,
    )]
    password: String,
}

#[derive(ToSchema, Debug, Serialize, Deserialize, Clone)]
pub struct UserCreationResponse {
    created: bool,
}


#[utoipa::path(
    request_body = UserCreationRequest,
    responses((
        status = 201,
        body = UserCreationResponse
    ),(
        status = BAD_REQUEST,
        description = "User could not be created",
        body = String
    ))
)]
#[post("/user")]
pub async fn create_user(request: Json<UserCreationRequest>) -> Result<Json<UserCreationResponse>, ApiError> {
    let (email, name, password) = (request.email.clone(), request.name.clone(), request.password.clone());
    let (password_hash, salt) = generate_password(password);
    db::user::validate_email_and_name(&email, &name).await?;
    db::user::create_user(email, name, password_hash, salt).await?;
    Ok(actix_web::web::Json(UserCreationResponse{created: true}))
}

pub fn generate_password(password: String) -> (String, Vec<u8>) {
    let salt = SaltString::generate(OsRng).as_str().as_bytes();
    let argon2 = Argon2::default();
    let mut hash: String;
    argon2.hash_password_into(&password.as_bytes(), &salt, &mut hash.as_bytes());
    (hash, salt.to_vec())
}