use axum::extract::{Query, Path};
use axum::response::Html;
use axum::{middleware, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;
use axum::routing::get;
use crate::login_api;
use crate::models::HelloParams;
use crate::rest_api::model::ModelController;
use crate::login_api::errors::Error;
use crate::rest_api::routes_tickets::routes_tickets;
use crate::static_routes::routes::routes_static;
use crate::auth_middleware::mw_auth::{mw_require_auth, mw_ctx_resolver};
use axum::response::IntoResponse;
use login_api::web::routes_login::routes_login;
use axum::response::Response;
use axum::middleware::{map_response, from_fn_with_state};
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;


pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let mc = ModelController::new().await?;

    let routes_apis = routes_tickets(mc.clone()) // the route layer ensures that the middleware only applies for this route 
    .route_layer(middleware::from_fn(mw_require_auth));

    let router = Router::new()
    .merge(routes_login())
    .merge(basic_routes())
    .nest("/api", routes_apis)
    .layer(map_response(main_response_mapper))
    .layer(from_fn_with_state(
        mc.clone(),
        mw_ctx_resolver

    ))
    .layer(CookieManagerLayer::new()) // layers get executed from bottom to top so if you want cookies they have to be below where you want them
    .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>().clone();
    let client_status_and_error = service_error.map(|se| se.client_status_and_error());
    let error_response = client_status_and_error
    .as_ref()
    .map(|(status_code, client_error)| {
        let client_error_body =json!({
        "error": {
            "type": client_error.as_ref(),
            "uuid": uuid.to_string()
        }});

        println!(" --> client_error_body: {}", client_error_body);
        (*status_code, Json(client_error_body)).into_response()
    });

    // todo: build server log line
    println!("  --> server log line {} service error {service_error:?}", uuid);

    error_response.unwrap_or(res)
}

fn basic_routes() -> Router {
    Router::new()
    .route("/", get(hello_handler))
    .route("/hello", get(hello_with_param_handler))
    .route("/hello/:name", get(hello_with_path_param_handler))
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn hello_with_param_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("<h1>Hello, {}!</h1>", name))
}

async fn hello_with_path_param_handler(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Hello, {name}!</h1>"))
}