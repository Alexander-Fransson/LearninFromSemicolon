use axum::{extract::State, response::{IntoResponse, Response}, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{to_value, Value};
use task_rpc::list_tasks;
use tracing::debug;

use crate::{ctx::Ctx, model::ModelManager, web::{Error, Result}};

mod task_rpc;

#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D
}

#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
    .route("/rpc", post(rpc_handler))
    .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>
) -> Response {
    _rpc_handler(ctx, mm, rpc_req).await.into_response()
}

async fn _rpc_handler(
    ctx: Ctx, 
    mm: ModelManager, 
    rpc_req: RpcRequest
) -> Result<Json<Value>> {

    let RpcRequest { 
        id: rpc_id, 
        method: rpc_method, 
        params: rpc_params 
    } = rpc_req;

    debug!("{:<12} _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        "create_task" => todo!(),
        "delete_task" => todo!(),
        "list_tasks" => list_tasks(ctx, mm).await.map(to_value)??,
        "update_task" => todo!(),
        _ => return Err(Error::RpcMethodNotFound(rpc_method))
    };

    todo!();
}