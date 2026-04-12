mod basic_info;
mod kind;
mod network_info;
mod pod_info;
mod state;

pub use basic_info::QuadletBasicInfo;
pub use kind::QuadletKind;
pub use network_info::NetworkInfo;
pub use pod_info::PodInfo;
pub use state::ActiveState;

#[derive(PartialEq, Eq)]
pub enum QuadletDetailedInfo {
    Pod(PodInfo),
    Network(NetworkInfo),
}

impl TryFrom<&QuadletBasicInfo> for QuadletDetailedInfo {
    type Error = std::io::Error;

    fn try_from(q: &QuadletBasicInfo) -> Result<Self, Self::Error> {
        match q.kind {
            QuadletKind::Container => todo!(),
            QuadletKind::Network => NetworkInfo::get(q).map(Self::Network),
            QuadletKind::Pod => PodInfo::get(q).map(Self::Pod),
            QuadletKind::Volume => todo!(),
            QuadletKind::Unknown => todo!(),
        }
    }
}
