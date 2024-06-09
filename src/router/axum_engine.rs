use super::interface::{AxumEngine, GymInterface};
use crate::errors::Result;
use async_trait::async_trait;

#[async_trait]
impl GymInterface for AxumEngine {}
