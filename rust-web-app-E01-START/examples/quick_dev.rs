#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:3000")?;

	hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);

	req_login.await?.print().await?;

	let req_create_tasks = hc.do_post("/api/rpc", json!({
		"id":2,
		"method":"create_task",
		"params": json!({
			"data": {
				"title": "test",
			}
		})

	}));

	req_create_tasks.await?.print().await?;


	let req_list_tasks = hc.do_post("/api/rpc", json!({
		"id":1,
		"method":"list_tasks",
	}));

	req_list_tasks.await?.print().await?;


	///////////////////
	/// used to test the logoff feature
	///////////////////////////////////
	
	// hc.do_get("/hello").await?.print().await?;

	// let req_logoff = hc.do_post("/api/logoff", json!({
	// 	"logoff": true
	// }));
	// req_logoff.await?.print().await?;

	// hc.do_get("/hello").await?.print().await?;


	Ok(())
}
