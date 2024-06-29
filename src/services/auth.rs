use crate::{
    config::db::DatabaseClient,
    errors::{self, Result},
    repo::{
        interface::DBInterface,
        model::{PayloadUserResponse, User},
    },
};

#[derive(Clone)]
pub struct AuthServices {
    pub repo: DatabaseClient,
}

impl AuthServices {
    pub async fn login(&self, email: String) -> Result<PayloadUserResponse> {
        let repo = self.repo.clone();

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
