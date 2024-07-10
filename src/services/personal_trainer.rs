use crate::{
    adapter::model::{Id, PersonalTrainer},
    errors::Result,
    repository::personal_trainer::PersonalTrainerRepository,
};

pub struct PersonalTrainerServices {
    pub repository: PersonalTrainerRepository,
}

impl PersonalTrainerServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, data: &PersonalTrainer) -> Result<Option<Id>> {
        let repo = &self.repository;

        let data_insert = repo.insert_data(data).await?;
        Ok(data_insert)
    }
}
