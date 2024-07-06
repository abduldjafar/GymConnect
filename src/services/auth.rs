use crate::{
    adapter::{interface::DBInterface, model::PayloadUserResponse},
    config::db::DatabaseClient,
    errors::{self, Result}, repository::user::UserRepository,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthServices {
    pub repo: Arc<DatabaseClient>,
    pub user_repository: UserRepository,
}

impl AuthServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn login(&self, email: String) -> Result<PayloadUserResponse> {
        let repo = &self.repo;

        let vect_data: Vec<PayloadUserResponse> = repo
            .select_where(
                "user".to_owned(),
                format!("email = '{}'", email),
                "*".to_string(),
            )
            .await?;

        if vect_data.is_empty() {
            return Err(errors::Error::DataNotAvaliable(String::from(
                "User not found",
            )));
        }

        let data = vect_data.get(0).unwrap().clone();

        Ok(data)
    }
}
