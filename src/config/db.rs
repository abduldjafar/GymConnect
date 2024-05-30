use std::error::Error;
use axum::async_trait;
use serde::Deserialize;
use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, sql::Thing, Surreal};

use crate::environment::Environment;

/*SurrealDB Struct,enum,trait Initialization*/
pub struct SurrealDb {
    pub client: Option<Surreal<Client>>,
}

pub struct DatabaseSource {
    pub db_type: DatabaseType,
}

pub enum DatabaseType {
    SurrealDB,
    // Add other database types here, e.g., Postgres
}

pub enum DatabaseConnection {
    Surreal(SurrealDb),
    // Add other database types here, e.g., Postgres(PostgresDb)
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[async_trait]
pub trait Initializable {
    async fn init(&self) -> Result<DatabaseConnection, Box<dyn Error>>;
}

#[async_trait]
pub trait Connection {
    fn ping(&self);
}


#[async_trait]
pub trait Sources {
    async fn connect(&mut self) -> Result<DatabaseConnection, Box<dyn Error>>;
}

// surrealDB implementation
#[async_trait]
impl Initializable for SurrealDb {
    async fn init(&self) -> Result<DatabaseConnection, Box<dyn Error>> {
        let env = Environment::new();
        let hostname = format!("{}:{}", env.db_host, env.db_port);
        let temp_client = Surreal::new::<Ws>(hostname).await?;

        temp_client.signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    
        temp_client.use_ns("test").use_db("test").await?;

        let client = Some(temp_client);
        Ok(DatabaseConnection::Surreal(SurrealDb { client }))
    }
}

impl Connection for SurrealDb {
    fn ping(&self) {
        if let Some(client) = &self.client {
            println!("{:?}",client);
        } else {
            println!("SurrealDb client is not connected.");
        }
    }
}


impl Connection for DatabaseConnection {
    fn ping(&self) {
        match self {
            DatabaseConnection::Surreal(surrealdb) => surrealdb.ping(),
            // Add other database types here
        }
    }
}


// general implementation
#[async_trait]
impl Sources for DatabaseSource {
    async fn connect(&mut self) -> Result<DatabaseConnection, Box<dyn Error>> {
        match &self.db_type {
            DatabaseType::SurrealDB => {
                let surrealdb = SurrealDb { client: None };
                surrealdb.init().await
            }
        }
    }
}