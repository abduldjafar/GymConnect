use chrono::Utc;

use crate::{
    adapter::model::{Gymnast, Id, PayloadGymnastRequest, PayloadGymnastResponse, User},
    errors::{self, Result},
    repository::{gymnast::GymnastRepository, user::UserRepository},
};

#[derive(Clone, Debug)]
pub struct GymnastServices {
    pub repository: GymnastRepository,
    pub user_repository: UserRepository,
}

impl GymnastServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_gymanst_user_empty(&self, user_id: &str) -> Result<(bool, Vec<Gymnast>)> {
        let data_exists = self
            .repository
            .is_gymnast_data_empty_by_user_id(user_id)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_user_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self
            .repository
            .is_gymnast_user_empty_by_email(&data.email)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_username_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self.user_repository.is_data_empty_by_username(data).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>> {

        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime(Utc::now());
        

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

        let insert_into_user_tb: Option<Id> = self.user_repository.insert_data(&data).await?;

        let user_id = insert_into_user_tb.unwrap();
        

        let (not_exists, _) = self
            .is_gymanst_user_empty(user_id.id.to_string().as_str())
            .await?;

        if !not_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let gymnast_data = Gymnast {
            id: None,
            user_id: Some(user_id.id),
            address: Some("".to_string()),
            sex:  Some("".to_string()),
            birth:  Some("".to_string()),
            phone:  Some("".to_string()),
            created_at: Some(time_now.clone()),
            updated_at: Some(time_now.clone()),
        };

        let insert_into_gym_tb: Option<Id> = self.repository.insert_data(&gymnast_data).await?;

        Ok(insert_into_gym_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn profile_details(&self, user_id: String) -> Result<PayloadGymnastResponse> {
        let (is_empty, temp_gymnast_user) = self.is_gymanst_user_empty(&user_id).await?;

        if is_empty {
            return Err(errors::Error::DataNotAvaliable(format!("{}", &user_id)));
        }

        let data_array: Vec<PayloadGymnastResponse> = temp_gymnast_user
            .into_iter()
            .map(|gymnast| PayloadGymnastResponse {
                id: gymnast.id.unwrap().to_string(),
                user_id: gymnast.user_id.unwrap().to_string(),
                address: gymnast.address,
                sex: gymnast.sex,
                birth: gymnast.birth,
                phone: gymnast.phone,
                created_at: gymnast.created_at,
                updated_at: gymnast.updated_at,
            })
            .collect();

        let data = match data_array.get(0).take() {
            Some(data) => data.to_owned(),
            None => {
                return Err(errors::Error::DataNotAvaliable(format!("{}", &user_id)));
            }
        };

        Ok(data)
    }

    pub async fn update_profile(
        &self,
        payload: &PayloadGymnastRequest,
        user_id: String,
    ) -> Result<()> {
        let (not_exists, existing_data) = self.is_gymanst_user_empty(&user_id).await?;

        if not_exists {
            return Err(errors::Error::DataNotAvaliable(format!("{}", user_id)));
        }

        let existing_record = &existing_data[0];
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime(Utc::now());

        let data = PayloadGymnastRequest {
            created_at: existing_record
                .created_at
                .clone()
                .or_else(|| Some(time_now.clone())),
            updated_at: Some(time_now),
            address: payload
                .address
                .clone()
                .or_else(|| existing_record.address.clone()),
            sex: payload.sex.clone().or_else(|| existing_record.sex.clone()),
            birth: payload
                .birth
                .clone()
                .or_else(|| existing_record.birth.clone()),
            phone: payload
                .phone
                .clone()
                .or_else(|| existing_record.phone.clone()),
        };

        let gymnast_id = existing_record.clone().id.unwrap().to_string();

        let update_data = self.repository.update_data(gymnast_id, &data).await?;

        if !update_data {
            return Err(errors::Error::DatabaseError(format!("{}", user_id)));
        }

        Ok(())
    }
}
