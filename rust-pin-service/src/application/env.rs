use crate::cipher::Algorithm;
use serde::Deserialize;
use std::fs;
use serde_yaml;

fn def_alg() -> Algorithm {
    Algorithm::Aes256Gcm
}

#[derive(Clone, Debug, Deserialize)]
pub struct CipherConf {
    pub key: String,
    #[serde(default = "def_alg")]
    pub alg: Algorithm,
    pub alg_str: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConf {
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegistryConf {
    pub hostname: String,
    pub ip_address: String,
    pub port: u16,
    pub retry_attempts: u16,
    pub data_center_info_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatasourceConf {
    pub hostname: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database_name: String,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
    pub max_idle_time: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppEnv {
    pub cipher: CipherConf,
    pub grpc: GrpcConf,
    pub registry: RegistryConf,
    pub app_name: String,
    pub datasource: DatasourceConf,
}

impl AppEnv {
    pub fn from(conf_path: &str) -> Self {
        println!("Reading conf from {conf_path}");
        
        let yaml = fs::read_to_string(conf_path)
            .expect("Unable to read config file!");
        
        let config: AppEnv = serde_yaml::from_str(&yaml)
            .expect("Unable to parse config file!");
        config
    }
}
