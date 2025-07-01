use reqwest::Client;
use uuid::Uuid;

use crate::application::{env::AppEnv, registry::{utils, DataCenterInfo, Instance, Port}};
 
pub struct EurekaRegisteryClient {
    instance_id: String,
    hostname: String,
    ip_address: String,
    eureka_port: u16,
    grpc_port: u16,
    app_name: String,
    data_center_info_name: String,
    client: Client,
    retry_attempts: u16,
}

impl EurekaRegisteryClient {

    pub fn new(env: &AppEnv) -> Self {
        Self { 
            instance_id: Uuid::new_v4().to_string(), 
            hostname: env.registry.hostname.clone(), 
            ip_address: env.registry.ip_address.clone(), 
            eureka_port: env.registry.port, 
            retry_attempts: env.registry.retry_attempts,
            grpc_port: env.grpc.port, 
            data_center_info_name: env.registry.data_center_info_name.clone(),  
            app_name: env.app_name.clone(), 
            client: Client::new()
        }
    }

    fn get_instance(&self) -> Instance {
        Instance {
            instance_id: self.instance_id.clone(),
            host_name: self.hostname.clone(),
            app: self.app_name.clone().to_uppercase(),
            ip_addr: self.ip_address.clone(),
            vip_address: self.app_name.clone().to_lowercase(),
            status: "UP".to_string(),
            port: Port { 
                port: self.grpc_port, 
                enabled: true 
            },
            data_center_info: DataCenterInfo {
                class: "com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo".into(),
                name: self.data_center_info_name.clone().into(),
            },
        }
    }

    pub fn start(&self) {
        self.register();
        self.heartbeat();
    }

    fn register(&self) {
        let client = self.client.clone(); 
        let app_name = self.app_name.clone();
        let hostname = self.hostname.clone();
        let instance = self.get_instance(); 
        let port = self.eureka_port;
        let attempts = self.retry_attempts;
        let eureka_base = format!("http://{}:{}/eureka", hostname, port);

        tokio::spawn(async move {    
            if let Err(err) = utils::register_in_eureka(&client, instance, app_name, eureka_base, attempts).await {
                eprintln!("FATAL ERROR... Eureka registration failed: {:?}", err);
            } else {
                println!("Successfully registered to Eureka...");
            }
        });
    }
    fn heartbeat(&self) {
        let client = self.client.clone(); 
        let app_name = self.app_name.clone();
        let hostname = self.hostname.clone();
        let instance_id = self.instance_id.clone(); 
        let port = self.eureka_port;
        let eureka_base = format!("http://{}:{}/eureka", hostname, port);

        tokio::spawn(async move {
            loop {
                let _ = utils::heartbeat(&client, &instance_id, &app_name, &eureka_base).await;
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            }
        });
    }
}