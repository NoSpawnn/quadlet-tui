use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::quadlet::types::QuadletBasicInfo;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct PodInfo {
    pub id: String,
    pub name: String,
    pub created: DateTime<Utc>,
    pub create_command: Vec<String>,
    pub exit_policy: String,
    pub state: String,
    pub hostname: String,
    pub labels: HashMap<String, String>,
    pub cgroup_parent: String,
    pub cgroup_path: String,
    #[serde(rename = "InfraContainerID")]
    pub infra_container_id: String,
    pub infra_config: HashMap<String, serde_json::Value>,
    pub containers: Vec<PodContainerInfo>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct PodContainerInfo {
    pub id: String,
    pub name: String,
    pub state: String,
}

impl PodInfo {
    pub fn get(q: &QuadletBasicInfo) -> std::io::Result<Self> {
        let cmd = std::process::Command::new("podman")
            .args(["inspect", "--type", "pod", "--format", "json", &q.name])
            .output()?;
        let res = serde_json::from_slice::<Vec<Self>>(&cmd.stdout)?;

        Ok(res.into_iter().next().unwrap())
    }
}
