use std::sync::Arc;

use super::{
    DBClient, GymnastId, GymnastModel, RepositoryGymnastRequest, RepositoryResult, UserModel,
};
use crate::adapter::interface::DBInterface as _;

#[derive(Clone, Debug)]
pub struct GymnastRepository {
    pub repo: Arc<DBClient>,
}

impl GymnastRepository {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_gymnast_data_empty_by_user_id(
        &self,
        user_id: &str,
    ) -> RepositoryResult<(bool, Vec<GymnastModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<GymnastModel> = repo
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
    pub async fn is_gymnast_user_empty_by_email(
        &self,
        email: &str,
    ) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let repo = self.repo.clone();

        let data_exists = {
            let data: Vec<UserModel> = repo
                .select_where(
                    "user".to_owned(),
                    format!("email = '{}' and user_type = 'gymnast'", email),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn insert_data(&self, data: &GymnastModel) -> RepositoryResult<Option<GymnastId>> {
        let repo = &self.repo;
        let insert_into_gym_tb: Option<GymnastId> =
            repo.insert_record(String::from("gymnast"), data).await?;
        Ok(insert_into_gym_tb)
    }

    pub async fn update_data(
        &self,
        gymnast_id: String,
        data: &RepositoryGymnastRequest,
    ) -> RepositoryResult<bool> {
        let repo = &self.repo;
        let update_data = repo
            .update_record(gymnast_id, "gymnast".to_string(), data)
            .await?;

        Ok(update_data)
    }
}
