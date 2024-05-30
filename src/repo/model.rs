use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

/* Struct representing a User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,            // Username of the user
    pub user_type: String,           // Type of the user (e.g., admin, regular user)
    pub email: String,               // Email of the user
    pub created_at: Option<Datetime>, // Timestamp when the user was created
    pub updated_at: Option<Datetime>, // Timestamp when the user was last updated
    pub password: String             // Password of the user
}
