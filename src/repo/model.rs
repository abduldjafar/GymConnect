use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Clone, Debug, PartialEq, Serialize,Deserialize)]
pub struct User {
    pub username: String,
    pub user_type: String,
    pub email: String,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
    pub password: String
}
