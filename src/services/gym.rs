use crate::{
    config::db::DatabaseClient,
    repo::{
        interface::DBInterface,
        model::{Gym, Id, User},
    },
};
use std::error::Error;

pub struct GymServices {
    pub repo: DatabaseClient,
}

impl GymServices {
    pub async fn register(&self, data: &User) -> Result<Option<Id>, Box<dyn Error>> {
        let repo = &self.repo;
        let insert_into_user_tb: Option<Id> =
            repo.insert_record(String::from("user"), data).await?;

        let gym_id = insert_into_user_tb.unwrap().id;

        let gym_data = Gym {
            user_id: std::option::Option::Some(gym_id),
            created_at: None,
            updated_at: None,
        };

        let insert_into_gym_tb: Option<Id> =
            repo.insert_record(String::from("gym"), &gym_data).await?;

        Ok(insert_into_gym_tb)
    }
}
