use axum::extract::{Query, Path};
use axum::response::Html;
use axum::Router;
use tokio::net::TcpListener;
use axum::routing::get;
use crate::login_api;
use crate::models::HelloParams;
use crate::rest_api::model::ModelController;
use crate::rest_api::routes_tickets::routes_tickets;
use crate::static_routes::routes::routes_static;
use axum::response::IntoResponse;
use login_api::web::routes_login::routes_login;
use axum::response::Response;
use axum::middleware::map_response;
use tower_cookies::CookieManagerLayer;

pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let mc = ModelController::new().await?;

    let router = Router::new()
    .merge(routes_login())
    .merge(basic_routes())
    .nest("/api", routes_tickets(mc.clone()))
    .layer(map_response(main_response_mapper))
    .layer(CookieManagerLayer::new()) // layers get executed from bottom to top so if you want cookies they have to be below where you want them
    .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");    
    res
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