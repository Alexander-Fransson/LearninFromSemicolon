use serde::{Deserialize, Serialize};

// to create in postgres cli
// CREATE TABLE users (
//     id SERIAL PRIMARY KEY,
//     name VARCHAR(255) NOT NULL,
//     occupation VARCHAR(255) NOT NULL,
//     email VARCHAR(255) NOT NULL,
//     phone VARCHAR(255) NOT NULL
// );

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    occupation: String,
    email: String,
    phone: String
}

pub struct UserInput {
    name: String,
    occupation: String,
    email: String,
    phone: String
}