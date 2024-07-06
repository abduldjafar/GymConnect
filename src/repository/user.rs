use std::sync::Arc;

use super::{DBClient, RepositoryResult, UserId, UserModel};
use crate::adapter::interface::DBInterface as _;

#[derive(Clone)]
pub struct UserRepository {
    pub repo: Arc<DBClient>,
}

impl UserRepository {
    #[tracing::instrument(err, skip_all)]
    pub async fn insert_data(&self, data: &UserModel) -> RepositoryResult<Option<UserId>> {
        let repo = &self.repo;
        let insert_into_user_tb: Option<UserId> =
            repo.insert_record(String::from("user"), data).await?;
        Ok(insert_into_user_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_email(
        &self,
        data: &UserModel,
    ) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<UserModel> = repo
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
    pub async fn is_data_empty_by_username(
        &self,
        data: &UserModel,
    ) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<UserModel> = repo
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
}
