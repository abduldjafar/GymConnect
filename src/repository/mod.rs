use crate::{
    adapter::model::{Gym, Id, PayloadGymRequest, User},
    config::db::DatabaseClient,
    errors::Result,
};

pub mod gym;
pub mod user;

type DBClient = DatabaseClient;
type RepositoryResult<T> = Result<T>;

type GymModel = Gym;
type GymId = Id;
type RepositoryGymRequest = PayloadGymRequest;

type UserModel = User;
type UserId = Id;
