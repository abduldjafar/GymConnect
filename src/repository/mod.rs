use crate::{
    adapter::model::{
        Gym, Gymnast, Id, PayloadGymRequest, PayloadGymnastRequest, PersonalTrainer, User,
    },
    config::db::DatabaseClient,
    errors::Result,
};

pub mod gym;
pub mod gymnast;
pub mod personal_trainer;
pub mod user;

type DBClient = DatabaseClient;
type RepositoryResult<T> = Result<T>;

type GymModel = Gym;
type GymId = Id;
type RepositoryGymRequest = PayloadGymRequest;

type GymnastModel = Gymnast;
type GymnastId = Id;
type RepositoryGymnastRequest = PayloadGymnastRequest;

type UserModel = User;
type UserId = Id;

type PersonalTrainerModel = PersonalTrainer;
type PersonalTrainerId = Id;
