use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::quadlet::types::QuadletBasicInfo;

#[derive(Deserialize, PartialEq, Eq)]
pub struct NetworkInfo {
    pub name: String,
    pub id: String,
    pub driver: String,
    pub network_interface: String,
    pub created: DateTime<Utc>,
    pub ipv6_enabled: bool,
    pub internal: bool,
    pub dns_enabled: bool,
    #[serde(rename = "network_dns_servers")]
    pub dns_servers: Vec<String>,
    pub containers: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
pub struct Subnet {
    pub subnet: String,
    pub gateway: String,
}

pub struct Container {
    pub name: String,
    pub interfaces: serde_json::Value,
}

impl NetworkInfo {
    pub fn get(q: &QuadletBasicInfo) -> std::io::Result<Self> {
        let cmd = std::process::Command::new("podman")
            .args(["inspect", "--type", "network", "--format", "json", &q.name])
            .output()?;
        let res = serde_json::from_slice::<Vec<Self>>(&cmd.stdout)?;

        Ok(res.into_iter().next().unwrap())
    }
}
