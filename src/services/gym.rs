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
    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>, Box<dyn Error>> {
        let repo = &self.repo;

        let insert_into_user_tb: Option<Id> =
            repo.insert_record(String::from("user"), data).await?;

        let gym_id = insert_into_user_tb.unwrap().id;

        let gym_data = Gym {
            user_id: std::option::Option::Some(gym_id),
            created_at: None,
            updated_at: None,
            address: String::from(""),
            owner_name: String::from(""),
            phone: 0,
        };

        let insert_into_gym_tb: Option<Id> =
            repo.insert_record(String::from("gym"), &gym_data).await?;

        Ok(insert_into_gym_tb)
    }

    pub async fn update_profile(
        &self,
        tb_name: String,
        data: &Gym,
        id: String,
    ) -> Result<(), Box<dyn Error>> {
        let repo = &self.repo;

        repo.update_record(id, tb_name, data).await?;

        Ok(())
    }
}
