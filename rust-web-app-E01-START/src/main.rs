#![allow(unused)] // For early development.

// region:    --- Modules

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;
mod crypt;

// #[cfg(test)] // For early development.
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use fields_macro::Fields;

pub trait Fields {
	fn struct_name(&self) -> &'static str;
	fn fields(&self) -> Vec<&'static str>;
	fn get_fields() -> Vec<&'static str>;
}

pub use config::config;

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::{middleware, Router};
use tracing_subscriber::EnvFilter;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {

	server_0().await;

	Ok(())
}

async fn server_0() -> Result<()> {

	// what does this do?
	tracing_subscriber::fmt()
	.without_time()
	.with_target(false)
	.with_env_filter(EnvFilter::from_default_env())
	.init();

	_dev_utils::dev_init().await;

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	// let routes_rpc = rpc::routes(mm.clone())
	//   .route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		// .nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server

	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
	.await
	.unwrap();

	println!("Listening on http://127.0.0.1:3000");

	axum::serve(listener, routes_all).await.unwrap();

	Ok(())
}
