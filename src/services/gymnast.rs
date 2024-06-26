use crate::{
    config::db::DatabaseClient,
    errors::{self, Result},
    repo::{
        interface::DBInterface,
        model::{Gymnast, Id, PayloadGymRequest, PayloadGymResponses, User},
    },
};
use chrono::prelude::*;


#[derive(Clone)]
pub struct GymnastServices {
    pub repo: DatabaseClient,
}

impl GymnastServices {
    async fn is_gymnast_user_empty(
        &self,
        repo: &DatabaseClient,
        gym_id: String,
    ) -> Result<(bool, Vec<Gymnast>)> {
        let data_exists = {
            let data: Vec<Gymnast> = repo
                .select_where(
                    "gymnast".to_owned(),
                    format!("user_id = '{}'", gym_id),
                    "*".to_string(),
                )
                .await?;

            (data.is_empty(), data)
        };

        Ok(data_exists)
    }
}