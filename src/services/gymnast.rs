use std::sync::Arc;

use crate::{
    config::db::DatabaseClient,
    errors::{self, Result},
    repo::{
        interface::DBInterface,
        model::{Gymnast, Id, User},
    },
};

#[derive(Clone,Debug)]
pub struct GymnastServices {
    pub repo: Arc<DatabaseClient>,
}

impl GymnastServices {

    #[tracing::instrument(err, skip_all)]
    pub async fn is_gymanst_user_empty(&self, user_id: String) -> Result<(bool, Vec<Gymnast>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<Gymnast> = repo
                .select_where(
                    "gymnast".to_owned(),
                    format!("user_id = '{}'", user_id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_user_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let repo = self.repo.clone();

        let data_exists = {
            let data: Vec<User> = repo
                .select_where(
                    "user".to_owned(),
                    format!(
                        "email = '{}' and user_type = '{}'",
                        data.email, data.user_type
                    ),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_username_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let repo = self.repo.clone();

        let data_exists = {
            let data: Vec<User> = repo
                .select_where(
                    "user".to_owned(),
                    format!("username = '{}'", data.username),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>> {
        let repo = &self.repo;

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

        let insert_into_user_tb: Option<Id> =
            repo.insert_record(String::from("user"), data).await?;

        let user_id = insert_into_user_tb.unwrap();

        let (not_exists, _) = self.is_gymanst_user_empty(user_id.id.to_string()).await?;

        if !not_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let gymnast_data = Gymnast {
            id: None,
            user_id: Some(user_id.id),
            address: String::from(""),
            sex: String::from(""),
            birth: String::from(""),
            phone: String::from(""),
            created_at: None,
            updated_at: None,
        };

        let insert_into_gym_tb: Option<Id> = repo
            .insert_record(String::from("gymnast"), &gymnast_data)
            .await?;

        Ok(insert_into_gym_tb)
    }
}
