
use crate::application::database::{DatabaseClient};
use crate::application::registry::client::EurekaRegisteryClient;
use crate::cipher::{Cipher, Algorithm};
use crate::cipher::aes::Aes256Cipher;
use crate::application::env::AppEnv;
use std::sync::Arc;

pub mod grpc;
pub mod env;
pub mod registry;
pub mod database;

#[derive(Clone)]
pub struct AppContext {
    pub cipher: Option<Arc<dyn Cipher + Send + Sync>>,
    pub env: AppEnv,
    pub db_client: DatabaseClient,
}

impl AppContext {
    pub fn new(env: &AppEnv) -> Self {
        let cipher: Option<Arc<dyn Cipher + Send + Sync>> = match env.cipher.alg {
            Algorithm::Aes256Gcm => Some(Arc::new(Aes256Cipher::new(env))),
            _ => None,
        };
        let db_client = DatabaseClient::new(env);

        Self {
            cipher,
            env: env.clone(),
            db_client: db_client
        }
    }
}

pub async fn boot() -> AppContext{

    let root_dir = std::env::current_dir().expect("Error"); 
    let config_path = root_dir.join("config.yml");
    let env = AppEnv::from(&config_path.to_str().unwrap());
    let mut context = AppContext::new(&env);
    let _ = context.db_client.init().await;

    let registy = EurekaRegisteryClient::new(&env);
    registy.start();


    grpc::run_grpc_server_bl(&context);
    context
}