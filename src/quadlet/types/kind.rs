use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum QuadletKind {
    Container,
    Network,
    Pod,
    Volume,
    #[default] // to make serde happy
    Unknown,
}

impl Display for QuadletKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                QuadletKind::Container => "Container",
                QuadletKind::Network => "Network",
                QuadletKind::Pod => "Pod",
                QuadletKind::Volume => "Volume",
                QuadletKind::Unknown => "Unknown",
            }
        )
    }
}

impl From<&str> for QuadletKind {
    fn from(value: &str) -> Self {
        match value {
            "container" => Self::Container,
            "network" => Self::Network,
            "pod" => Self::Pod,
            "volume" => Self::Volume,
            _ => unreachable!(),
        }
    }
}
