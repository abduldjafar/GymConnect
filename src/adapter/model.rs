use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

/* Struct representing a User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]

pub struct User {
    pub username: String,             // Username of the user
    pub user_type: String,            // Type of the user (e.g., admin, regular user)
    pub email: String,                // Email of the user
    pub created_at: Option<Datetime>, // Timestamp when the user was created
    pub updated_at: Option<Datetime>, // Timestamp when the user was last updated
    pub password: String,             // Password of the user
}

/* Struct representing a Gym User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gym {
    pub id: Option<Thing>,
    pub user_id: Option<Thing>,
    pub address: String,
    pub owner_name: String,
    pub phone: String,
    pub created_at: Option<Datetime>, // Timestamp when the user was created
    pub updated_at: Option<Datetime>, // Timestamp when the user was last updated
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Id {
    pub id: Thing,
}

/* Struct representing a User in the database */
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PayloadUser {
    pub username: String, // Username of the user
    pub email: String,    // Email of the user
    pub password: String, // Password of the user
}

/* Struct for deserialization of records */
#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadIdResponses {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymResponses {
    pub id: String,
    pub address: String,
    pub owner_name: String,
    pub phone: String,
    pub created_at: Option<Datetime>, // Timestamp when the user was created
    pub updated_at: Option<Datetime>, // Timestamp when the user was last updated
    pub user_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymRequest {
    pub address: Option<String>,
    pub owner_name: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
    pub user_id: Option<Thing>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PayloadUserResponse {
    pub id: Thing,
    pub username: String,  // Username of the user
    pub user_type: String, // Type of the user (e.g., admin, regular user)
    pub email: String,     // Email of the user
    pub password: String,  // Password of the user
}

/* Struct representing a Gym User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gymnast {
    pub id: Option<Thing>,
    pub user_id: Option<Thing>,
    pub address: Option<String>,
    pub sex: Option<String>,
    pub birth: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymnastRequest {
    pub address: Option<String>,
    pub sex: Option<String>,
    pub birth: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

/* Struct representing a Gym User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymnastResponse {
    pub id: String,
    pub user_id: String,
    pub address: Option<String>,
    pub sex: Option<String>,
    pub birth: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
