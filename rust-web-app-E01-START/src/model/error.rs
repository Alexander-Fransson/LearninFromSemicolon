use serde::Serialize;
use crate::{crypt, model::store};
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	Crypt(crypt::Error),
	Store(store::Error),
	Sqlx(
		#[serde_as(as = "DisplayFromStr")]
		sqlx::Error
	),
	EntityNotFound { entity: &'static str, id: i64 },
}

impl From<crypt::Error> for Error {
	fn from(err: crypt::Error) -> Self {
		Self::Crypt(err)
	}
}

impl From<sqlx::Error> for Error {
	fn from(err: sqlx::Error) -> Self {
		Self::Sqlx(err)
	}
}

impl From<store::Error> for Error {
	fn from(err: store::Error) -> Self {
		Error::Store(err)
	}
}

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
