mod basic_info;
mod container_info;
mod kind;
mod network_info;
mod pod_info;
mod state;

pub use basic_info::QuadletBasicInfo;
pub use container_info::ContainerInfo;
pub use kind::QuadletKind;
pub use network_info::NetworkInfo;
pub use pod_info::PodInfo;
pub use state::ActiveState;

#[derive(PartialEq, Eq)]
pub enum QuadletDetailedInfo {
    Pod(PodInfo),
    Network(NetworkInfo),
    Container(ContainerInfo),
}

impl TryFrom<&QuadletBasicInfo> for QuadletDetailedInfo {
    type Error = std::io::Error;

    fn try_from(q: &QuadletBasicInfo) -> Result<Self, Self::Error> {
        match q.kind {
            QuadletKind::Container => ContainerInfo::get(q).map(Self::Container),
            QuadletKind::Network => NetworkInfo::get(q).map(Self::Network),
            QuadletKind::Pod => PodInfo::get(q).map(Self::Pod),
            QuadletKind::Volume => todo!(),
            QuadletKind::Unknown => todo!(),
        }
    }
}
