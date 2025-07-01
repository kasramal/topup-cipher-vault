use serde::Serialize;

pub mod utils;
pub mod client;

#[derive(Serialize)]
pub struct Instance {
    #[serde(rename = "hostName")]
    host_name: String,

    #[serde(rename = "app")]
    app: String,

    #[serde(rename = "ipAddr")]
    ip_addr: String,

    #[serde(rename = "status")]
    status: String,

    #[serde(rename = "port")]
    port: Port,

    #[serde(rename = "vipAddress")]
    vip_address: String,

    #[serde(rename = "dataCenterInfo")]
    data_center_info: DataCenterInfo,

    #[serde(rename = "instanceId")]
    instance_id: String,
}

#[derive(Serialize)]
struct Port {
    #[serde(rename = "$")]
    port: u16,
    #[serde(rename = "@enabled")]
    enabled: bool,
}

#[derive(Serialize)]
struct DataCenterInfo {
    #[serde(rename = "@class")]
    class: String,
    name: String,
}

#[derive(Serialize)]
struct RegisterRequest {
    instance: Instance,
}