use std::sync::Arc;

use crate::{
    adapter::model::{Gym, Id, PayloadGymRequest, PayloadGymResponses, User},
    config::db::DatabaseClient,
    errors::{self, Result},
    repository::{gym::GymRepository, user::UserRepository},
};
use chrono::prelude::*;

#[derive(Clone)]
pub struct GymServices {
    pub repo: Arc<DatabaseClient>,
    pub gym_repository: GymRepository,
    pub user_repository: UserRepository,
}

impl GymServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_gym_user_empty(&self, user_id: String) -> Result<(bool, Vec<Gym>)> {
        let data_exists = self.gym_repository.is_gym_data_empty(&user_id).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_user_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self.user_repository.is_data_empty_by_email(data).await?;

        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_username_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self.user_repository.is_data_empty_by_username(data).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>> {
        let (is_user_empty, _) = self.is_user_empty(data).await?;
        if !is_user_empty {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let (is_username_empty, _) = self.is_username_empty(data).await?;
        if !is_username_empty {
            return Err(errors::Error::DataExist(format!(
                "username:{}",
                data.username
            )));
        }

        let insert_into_user_tb: Option<Id> = self.user_repository.insert_data(data).await?;

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

        let insert_into_gym_tb: Option<Id> = self.gym_repository.insert_data(&gym_data).await?;

        Ok(insert_into_gym_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn profile_details(&self, user_id: String) -> Result<PayloadGymResponses> {
        let temp_data: Vec<Gym> = self.gym_repository.get_details(&user_id).await?;

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
        let (not_exists, existing_data) = self.gym_repository.is_gym_data_empty(&user_id).await?;

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

        let update_data = self.gym_repository.update_data(gym_id, &data).await?;

        if !update_data {
            return Err(errors::Error::DatabaseError(format!("{}", user_id)));
        }

        Ok(())
    }
}
