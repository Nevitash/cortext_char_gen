pub mod user;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use utoipa::OpenApi;
use thiserror::Error;

use crate::db::DataError;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Cortex Prime character generator API",
        description = "Enables clients to anonymously check whether their credentials have been involved in a known data breach.",
        version = "0.0.1"
    ),
    components(schemas(user::UserCreationRequest, user::UserCreationResponse))
)]
pub struct ApiDocs;

impl ApiDocs {
    pub fn generator() -> String {
        ApiDocs::openapi().to_pretty_json().unwrap()
    }
}
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Email {0} already in use.")]
    EmailAlreadyInUse(String),
    #[error("Name {0} already in use.")]
    NameAlreadyInUse(String),
    #[error("Our database is unreachable atm. Please try again later.")]
    DatabaseTimeout(),
    #[error("Internal Server Error")]
    InternalServerError(Box<dyn std::error::Error>),
}

impl From<DataError> for ApiError {
    fn from(data_error: DataError) -> Self {
        match data_error {
            DataError::NameMustBeUnique(name) => ApiError::NameAlreadyInUse(name),
            DataError::EmailMustBeUnique(email) => ApiError::EmailAlreadyInUse(email),
            DataError::ConnectionError() => ApiError::DatabaseTimeout(),
            DataError::Unmapped(error) => ApiError::InternalServerError(Box::new(error)),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ApiError::EmailAlreadyInUse(..) => StatusCode::BAD_REQUEST,
            ApiError::NameAlreadyInUse(..) => StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type("text/plain")
            .body(self.to_string())
    }
}