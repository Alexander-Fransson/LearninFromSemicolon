use axum::{Extension, Router};
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
mod user_services;
mod secret_consts;

#[tokio::main]
    async fn main() {
    // rust
    // postgres
    // docker & kubernetes

    // to open postgres cli $ sudo -u user_name ("t.ex postgres") psql
    // to create database $ create database [name]
    // to connect to database $ \c [name]
    // to grant the user you are using all privaliges on the database $ grant all privileges on database [name] to [user]
    // to grant full access to table $ grant all privileges on table [table] to [user]
    
    
    let service = user_services::UserService::new().await.unwrap();

    let app = Router::new()
    .route("/users", get(list_users))
    .route("/users/:id", get(get_user_by_id))
    .route("/users", post(create_user))
    .route("/users/:id", put(update_user))
    .route("/users/:id", delete(delete_user))
    .layer(Extension(service));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
    .await
    .unwrap();
}
