use std::path::PathBuf;

use serde::Deserialize;

use crate::quadlet::types::{
    kind::QuadletKind,
    state::{ActiveState, Status, SubState},
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
/// Info from `podman quadlet list`
pub struct QuadletBasicInfo {
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(rename = "Name")]
    pub file_name: String,
    pub unit_name: String,
    #[serde(rename = "Path")]
    pub unit_file_path: PathBuf,
    pub status: Status,
    pub app: String,
    #[serde(skip_deserializing)]
    pub kind: QuadletKind,
}

impl QuadletBasicInfo {
    pub fn running(&self) -> bool {
        matches!(
            self.status.active,
            ActiveState::Active | ActiveState::Activating
        ) && matches!(self.status.sub, SubState::Running)
    }
}
