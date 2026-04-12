use std::fmt::Display;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub active: ActiveState,
    pub sub: SubState,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.active, self.sub)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveState {
    Active,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Reloading,
    Maintenance,
    Refreshing,
    Unknown(String),
}

impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ActiveState::Active => "active",
                ActiveState::Inactive => "inactive",
                ActiveState::Failed => "failed",
                ActiveState::Activating => "activating",
                ActiveState::Deactivating => "deactivating",
                ActiveState::Reloading => "reloading",
                ActiveState::Maintenance => "maintenance",
                ActiveState::Refreshing => "refreshing",
                ActiveState::Unknown(s) => s,
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubState {
    Running,
    Exited,
    Dead,
    Unknown(String),
}

impl Display for SubState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SubState::Running => "running",
                SubState::Exited => "exited",
                SubState::Dead => "dead",
                SubState::Unknown(s) => s,
            }
        )
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let mut parts = s.splitn(2, '/');

        let active = match parts.next().unwrap_or("") {
            "active" => ActiveState::Active,
            "inactive" => ActiveState::Inactive,
            "failed" => ActiveState::Failed,
            "activating" => ActiveState::Activating,
            "deactivating" => ActiveState::Deactivating,
            "reloading" => ActiveState::Reloading,
            "maintenance" => ActiveState::Maintenance,
            "refreshing" => ActiveState::Refreshing,
            other => ActiveState::Unknown(other.to_string()),
        };

        let sub = match parts.next().unwrap_or("") {
            "running" => SubState::Running,
            "exited" => SubState::Exited,
            "dead" => SubState::Dead,
            other => SubState::Unknown(other.to_string()),
        };

        Ok(Status { active, sub })
    }
}
