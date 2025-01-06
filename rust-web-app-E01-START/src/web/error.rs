use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Login
	LoginFail,
	LoginFailUserNotFound,
	LoginFailUserHasNoPwd{user_id: i64},
	LoginFailPwdNotMatching{user_id: i64},

	// Rpc
	RpcMethodNotFound(String),
	RpcMissingParams{rpc_method: String},
	RpcFailedJsonParams{rpc_method: String},

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),

	// -- Modules
	Model(model::Error),

	// -- Crypt
	Crypt(crypt::Error),

	// -- External modules
	SerdeJson(String)
}

impl From<crypt::Error> for Error {
	fn from(error: crypt::Error) -> Self {
		Self::Crypt(error)
	}
}

impl From<model::Error> for Error {
	fn from(error: model::Error) -> Self {
		Self::Model(error)
	}
}


impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Self {
		Self::SerdeJson(error.to_string())
	}
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!("{:<12} - model::Error {self:?}", "INTO_RES");

		/////////////////////////
		/// For some reason the error does not change, it is suposed to take the self and return the coresponding client error, maybe I should look into that in itself in the future

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self.to_string());

		response
	}
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::Error::*;

		#[allow(unreachable_patterns)]
		match self {
			// -- Login

			LoginFailUserNotFound |
			LoginFailUserHasNoPwd{..} |
			LoginFailPwdNotMatching{..} => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}
			
			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Model
			Model(model::Error::EntityNotFound { entity, id }) => (
				StatusCode::NOT_FOUND,
				ClientError::ENTITY_NOT_FOUND { entity, id: *id },
			),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

	SERVICE_ERROR,
}
// endregion: --- Client Error
