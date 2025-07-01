use mongodb::{
    Client,
    options::{ClientOptions},
};
use std::{time::Duration};

use crate::application::{
    database::utils::build_mongo_uri,
    env::{AppEnv, DatasourceConf}
};

pub mod utils;

#[derive(Clone)]
pub struct DatabaseClient {
    pub conf: DatasourceConf,
    pub client: Option<Client>,
}

impl DatabaseClient {
    pub fn new(env: &AppEnv) -> Self {
        Self {
            conf: env.datasource.clone(),
            client: None,
        }
    }

    pub async fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = build_mongo_uri(
            &self.conf.hostname,
            self.conf.port,
            &self.conf.database_name,
            self.conf.username.as_deref(),
            self.conf.password.as_deref(),
        );

        let mut client_options = ClientOptions::parse(&url).await?;

        client_options.max_pool_size = Some(self.conf.max_pool_size.unwrap_or(50));
        client_options.min_pool_size = Some(self.conf.min_pool_size.unwrap_or(10));
        client_options.max_idle_time =
            Some(Duration::from_secs(self.conf.max_idle_time.unwrap_or(300)));

        let client = Client::with_options(client_options)?;
        self.client = Some(client);

        Ok(())
    }

    pub fn db(&self) -> mongodb::Database {
        self.client
            .as_ref()
            .expect("Client not initialized properly!")
            .database(&self.conf.database_name)
    }
}
