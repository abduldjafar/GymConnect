use serde_json::Value;

use crate::{
    config::db::DatabaseClient,
    errors::{self, Result},
    repo::{
        interface::DBInterface,
        model::{Gym, Id, PayloadGymResponses, User},
    },
};

#[derive(Clone)]
pub struct GymServices {
    pub repo: DatabaseClient,
}

impl GymServices {
    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>> {
        let repo = &self.repo;

        let insert_into_user_tb: Option<Id> =
            repo.insert_record(String::from("user"), data).await?;

        let gym_id = insert_into_user_tb.unwrap().id;

        let data_exists = {
            let data: Vec<Value> = repo
                .select_where(
                    "gym".to_owned(),
                    format!("user_id = '{}'", gym_id.to_string()),
                    "user_id".to_string(),
                )
                .await?;
            data.is_empty()
        };

        if !data_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let gym_data = Gym {
            id: None,
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

    pub async fn profile_details(
        &self,
        gym_id: String,
    ) -> Result<Option<Vec<PayloadGymResponses>>> {
        let repo = &self.repo;

        let temp_data: Vec<Gym> = repo
            .select_where(
                "gym".to_owned(),
                format!("user_id = '{}'", gym_id.to_string()),
                "*".to_string(),
            )
            .await?;

        let data: Vec<PayloadGymResponses> = temp_data
            .into_iter()
            .map(|x| PayloadGymResponses {
                id: x.id.unwrap().to_string(),
                address: x.address,
                owner_name: x.owner_name,
                phone: x.phone,
                created_at: x.created_at,
                updated_at: x.updated_at,
                user_id: x.user_id.unwrap().to_string(),
            })
            .collect();

        Ok(Some(data))
    }

    pub async fn update_profile(&self, tb_name: String, data: &Gym, id: String) -> Result<()> {
        let repo = &self.repo;

        repo.update_record(id, tb_name, data).await?;

        Ok(())
    }
}
