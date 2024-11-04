use axum::Router;
use axum::routing::{get, post, put, delete};
use controller::{
    create_user, 
    get_user_by_id, 
    list_users, 
    update_user, 
    delete_user
};

mod model;
mod controller;

#[tokio::main]
    async fn main() {
    // rust
    // postgres
    // docker & kubernetes

    //to open postgres cli $ sudo -u user_name ("t.ex postgres") psql
    
    let app = Router::new()
    .route("/users", get(list_users))
    .route("/users/:id", get(get_user_by_id))
    .route("/users", post(create_user))
    .route("/users/:id", put(update_user))
    .route("/users/:id", delete(delete_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

    axum::serve(listener, app)
    .await
    .unwrap();
}
