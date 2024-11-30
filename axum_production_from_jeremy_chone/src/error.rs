use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Serialize, Debug, Clone, strum_macros::AsRefStr)]
#[serde(tag="type", content="error")]
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound {id: i32},
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExtension
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // member, never ever pass on your server error to your client
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    #[allow(unreachable_patterns)]
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL),
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExtension => (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),
            Self::TicketDeleteFailIdNotFound {..} => (StatusCode::NOT_FOUND, ClientError::SERVICE_ERROR),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR) // not needed but some may argue that it is good to profe the future
        }   
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self)    
    }
}

impl std::error::Error for Error {}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR
}