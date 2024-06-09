use crate::{errors::Result, services::gym::GymServices};
use async_trait::async_trait;

pub struct GymRouter {
    pub gym_services: GymServices,
}
