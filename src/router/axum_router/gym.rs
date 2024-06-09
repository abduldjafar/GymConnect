use crate::errors::Result;
use crate::repo::model::{PayloadUser, User};
use crate::router::interface::GymRouter;
use crate::services::gym::GymServices;
use axum::Json;
use serde_json::Value;

impl GymRouter {
    async fn register_gym_user(&self, payload: Json<PayloadUser>) -> Result<Json<Value>> {
        let gym_services = &self.gym_services;

        let user = User {
            username: todo!(),
            user_type: todo!(),
            email: todo!(),
            created_at: todo!(),
            updated_at: todo!(),
            password: todo!(),
        };

        let user_id = gym_services.register_profile(&user).await?;
    }
}
