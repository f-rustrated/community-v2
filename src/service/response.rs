use serde::Serialize;
use serde_json::Value;

// TODO define service error for fallible service operation
#[derive(Debug, Serialize)]
pub enum BaseError {
    DatabaseError,
    NotFound,
    ConstraintError,
    TransactionError,
    InternalError,
}

#[derive(Debug, Serialize)]
pub enum ServiceError {
    BaseError(BaseError),
    UnAuthorized(String),
    AuthenticationError(String),
    HashLibError(String),
    JWTError(String),
}

impl From<BaseError> for ServiceError {
    fn from(value: BaseError) -> Self {
        ServiceError::BaseError(value)
    }
}

#[derive(Serialize)]
pub enum ApplicationResponse {
    String(String),
    I64(i64),
    Json(Value),
    Empty(()),
}

impl From<String> for ApplicationResponse {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Value> for ApplicationResponse {
    fn from(value: Value) -> Self {
        Self::Json(value)
    }
}
