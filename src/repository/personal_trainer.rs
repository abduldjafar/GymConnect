use std::sync::Arc;

use super::{DBClient, PersonalTrainerId, PersonalTrainerModel, RepositoryResult};
use crate::adapter::interface::DBInterface as _;

pub struct PersonalTrainerRepository {
    pub repo: Arc<DBClient>,
}

impl PersonalTrainerRepository {
    pub async fn insert_data(
        &self,
        data: &PersonalTrainerModel,
    ) -> RepositoryResult<Option<PersonalTrainerId>> {
        let repo = &self.repo;
        let insert_into_gym_tb: Option<PersonalTrainerId> = repo
            .insert_record(String::from("personal_trainer"), data)
            .await?;
        Ok(insert_into_gym_tb)
    }
}
