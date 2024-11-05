use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

/* to create in postgres cli
    CREATE TABLE users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        occupation VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL,
        phone VARCHAR(255) NOT NULL
    ); 
*/

// see tables with $ \dt

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String
}


#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub occupation: String,
    pub email: String,
    pub phone: String
}