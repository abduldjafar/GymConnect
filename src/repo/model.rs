use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User<'a> {
    pub username: &'a str,
    pub user_type: &'a str,
    pub email: &'a str,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
    pub password: &'a str
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
