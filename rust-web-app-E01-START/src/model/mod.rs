//! Model layer 
//! Design:
//!  - the model layer normalizes the applications datatype structure and access
//!  - all application code data access must go through model layer
//!  - model manager holds the internal state/resources model controller needs to access data
//!    f.ex db_pool, s3 client and redis client
//!  - model controllers implement crud and such	

mod base;
mod error;
mod store;
pub mod task;

use store::new_db_pool;

pub use self::error::{Error, Result};
use self::store::DB;

// Aka model controller ctx?
#[derive(Clone)]
pub struct ModelManager {
	db:DB,
}

impl ModelManager {
	// 
	pub async fn new() -> Result<Self> {
		let db =  new_db_pool().await?;
		// FIXME - TBC
		Ok(ModelManager {db})
	}

	// returns the reference sql db pool only for the model layer
	pub (in crate::model) fn db(&self) -> &DB {
		&self.db
	}
}
