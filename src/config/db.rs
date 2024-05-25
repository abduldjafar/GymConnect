use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};

use crate::environment::Environment;

pub struct Db {
    pub environment: Environment,
}

impl Db {
    pub async fn init(self) ->  Result<Surreal<Client>,Box<dyn std::error::Error> > {
        let env = self.environment;
        let hostname =format!("{}:{}",env.db_host,env.db_port);
        let connection = Surreal::new::<Ws>(hostname).await?;
        Ok(connection)
    }


}
