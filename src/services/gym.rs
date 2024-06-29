use crate::{
    config::db::DatabaseClient,
    errors::{self, Result},
    repo::{
        interface::DBInterface,
        model::{Gym, Id, PayloadGymRequest, PayloadGymResponses, User},
    },
};
use chrono::prelude::*;

#[derive(Clone)]
pub struct GymServices {
    pub repo: DatabaseClient,
}

impl GymServices {
    async fn is_gym_user_empty(&self, gym_id: String) -> Result<(bool, Vec<Gym>)> {
        let repo = self.repo.clone();

        let data_exists = {
            let data: Vec<Gym> = repo
                .select_where(
                    "gym".to_owned(),
                    format!("user_id = '{}'", gym_id),
                    "*".to_string(),
                )
                .await?;

            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>> {
        let repo = &self.repo;

        let insert_into_user_tb: Option<Id> =
            repo.insert_record(String::from("user"), data).await?;

        let user_id = insert_into_user_tb.unwrap();

        let (not_exists, _) = self.is_gym_user_empty(user_id.id.to_string()).await?;

        if !not_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let gym_data = Gym {
            id: None,
            user_id: std::option::Option::Some(user_id.id),
            created_at: None,
            updated_at: None,
            address: String::from(""),
            owner_name: String::from(""),
            phone: String::from(""),
        };

        let insert_into_gym_tb: Option<Id> =
            repo.insert_record(String::from("gym"), &gym_data).await?;

        Ok(insert_into_gym_tb)
    }

    pub async fn profile_details(&self, user_id: String) -> Result<PayloadGymResponses> {
        let repo = &self.repo;

        let temp_data: Vec<Gym> = repo
            .select_where(
                "gym".to_owned(),
                format!("user_id = '{}'", user_id.to_string()),
                "*".to_string(),
            )
            .await?;

        let data_array: Vec<PayloadGymResponses> = temp_data
            .into_iter()
            .map(|gym| PayloadGymResponses {
                id: gym.id.unwrap().to_string(),
                address: gym.address,
                owner_name: gym.owner_name,
                phone: gym.phone,
                created_at: gym.created_at,
                updated_at: gym.updated_at,
                user_id: gym.user_id.unwrap().to_string(),
            })
            .collect();

        let data = match data_array.get(0).take() {
            Some(data) => data.to_owned(),
            None => {
                return Err(errors::Error::DataNotAvaliable(format!("{}", user_id)));
            }
        };

        Ok(data)
    }

    pub async fn update_profile(&self, payload: &PayloadGymRequest, user_id: String) -> Result<()> {
        let repo = &self.repo;

        let (not_exists, existing_data) = self.is_gym_user_empty(user_id.clone()).await?;

        if not_exists {
            return Err(errors::Error::DataNotAvaliable(format!("{}", user_id)));
        }

        let existing_record = &existing_data[0];
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime(Utc::now());

        let data = PayloadGymRequest {
            address: payload
                .address
                .clone()
                .or_else(|| Some(existing_record.address.clone())),
            owner_name: payload
                .owner_name
                .clone()
                .or_else(|| Some(existing_record.owner_name.clone())),
            phone: payload
                .phone
                .clone()
                .or_else(|| Some(existing_record.phone.clone())),
            created_at: existing_record
                .created_at
                .clone()
                .or_else(|| Some(time_now.clone())),
            updated_at: Some(time_now),
            user_id: existing_record.user_id.clone(),
        };

        let gym_id = existing_record.clone().id.unwrap().to_string();

        let update_data = repo.update_record(gym_id, "gym".to_string(), &data).await?;

        if !update_data {
            return Err(errors::Error::DatabaseError(format!("{}", user_id)));
        }

        Ok(())
    }
}
