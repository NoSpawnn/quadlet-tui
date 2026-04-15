use serde::Deserialize;
use std::collections::HashMap;

use crate::quadlet::types::QuadletBasicInfo;

impl ContainerInfo {
    pub fn get(q: &QuadletBasicInfo) -> std::io::Result<Self> {
        let cmd = std::process::Command::new("podman")
            .args([
                "inspect",
                "--type",
                "container",
                "--format",
                "json",
                &q.name,
            ])
            .output()?;
        let res = serde_json::from_slice::<Vec<Self>>(&cmd.stdout)?;

        Ok(res.into_iter().next().unwrap())
    }
}

use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub id: String,
    pub created: String,
    pub path: String,
    pub args: Vec<String>,
    pub state: State,
    pub image: String,
    pub image_digest: String,
    pub image_name: String,
    pub rootfs: String,
    pub pod: String,
    pub resolv_conf_path: String,
    pub hostname_path: String,
    pub hosts_path: String,
    pub static_dir: String,
    #[serde(rename = "OCIConfigPath")]
    pub ociconfig_path: String,
    #[serde(rename = "OCIRuntime")]
    pub ociruntime: String,
    pub conmon_pid_file: String,
    pub pid_file: String,
    pub name: String,
    pub restart_count: i64,
    pub driver: String,
    pub mount_label: String,
    pub process_label: String,
    pub app_armor_profile: String,
    pub effective_caps: Value,
    pub bounding_caps: Vec<String>,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Vec<Value>,
    pub graph_driver: GraphDriver,
    pub mounts: Vec<Value>,
    pub dependencies: Vec<Value>,
    pub network_settings: NetworkSettings,
    pub namespace: String,
    pub is_infra: bool,
    pub is_service: bool,
    pub kube_exit_code_propagation: String,
    #[serde(rename = "lockNumber")]
    pub lock_number: i64,
    pub config: Config,
    pub host_config: HostConfig,
    pub use_image_hosts: bool,
    pub use_image_hostname: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub oci_version: String,
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oomkilled: bool,
    pub dead: bool,
    pub pid: i64,
    pub conmon_pid: i64,
    pub exit_code: i64,
    pub error: String,
    pub started_at: String,
    pub finished_at: String,
    pub health: Health,
    pub cgroup_path: String,
    pub checkpointed_at: String,
    pub restored_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Health {
    pub status: String,
    pub failing_streak: i64,
    pub log: Vec<Log>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Log {
    pub start: String,
    pub end: String,
    pub exit_code: i64,
    pub output: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GraphDriver {
    pub name: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub lower_dir: String,
    pub merged_dir: String,
    pub upper_dir: String,
    pub work_dir: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,
    pub mac_address: String,
    pub bridge: String,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    pub hairpin_mode: bool,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6prefix_len: i64,
    pub ports: HashMap<String, Option<Vec<Port>>>,
    pub sandbox_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub hostname: String,
    pub domainname: String,
    pub user: String,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub attach_stderr: bool,
    pub tty: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub env: Vec<String>,
    pub cmd: Vec<String>,
    pub image: String,
    pub volumes: Value,
    pub working_dir: String,
    pub entrypoint: Vec<String>,
    pub on_build: Value,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
    pub stop_signal: String,
    pub healthcheck: Healthcheck,
    pub healthcheck_on_failure_action: String,
    pub health_log_destination: String,
    pub healthcheck_max_log_count: i64,
    pub healthcheck_max_log_size: i64,
    pub create_command: Vec<String>,
    pub umask: String,
    pub timeout: i64,
    pub stop_timeout: i64,
    pub passwd: bool,
    #[serde(rename = "sdNotifyMode")]
    pub sd_notify_mode: String,
    #[serde(rename = "sdNotifySocket")]
    pub sd_notify_socket: String,
    pub exposed_ports: HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Healthcheck {
    pub test: Vec<String>,
    pub interval: i64,
    pub timeout: i64,
    pub retries: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub binds: Vec<Value>,
    pub cgroup_manager: String,
    pub cgroup_mode: String,
    #[serde(rename = "ContainerIDFile")]
    pub container_idfile: String,
    pub log_config: LogConfig,
    pub network_mode: String,
    pub port_bindings: HashMap<String, Vec<Port>>,
    pub restart_policy: RestartPolicy,
    pub auto_remove: bool,
    pub auto_remove_image: bool,
    pub annotations: HashMap<String, String>,
    pub volume_driver: String,
    pub volumes_from: Value,
    pub cap_add: Vec<Value>,
    pub cap_drop: Vec<Value>,
    pub dns: Vec<Value>,
    pub dns_options: Vec<Value>,
    pub dns_search: Vec<Value>,
    pub extra_hosts: Vec<Value>,
    pub hosts_file: String,
    pub group_add: Vec<Value>,
    pub ipc_mode: String,
    pub cgroup: String,
    pub cgroups: String,
    pub links: Value,
    pub oom_score_adj: i64,
    pub pid_mode: String,
    pub privileged: bool,
    pub publish_all_ports: bool,
    pub readonly_rootfs: bool,
    pub security_opt: Vec<Value>,
    pub tmpfs: serde_json::Value,
    #[serde(rename = "UTSMode")]
    pub utsmode: String,
    pub userns_mode: String,
    #[serde(rename = "IDMappings")]
    pub idmappings: Idmappings,
    pub shm_size: i64,
    pub runtime: String,
    pub console_size: Vec<i64>,
    pub isolation: String,
    pub cpu_shares: i64,
    pub memory: i64,
    pub nano_cpus: i64,
    pub cgroup_parent: String,
    pub blkio_weight: i64,
    pub blkio_weight_device: Value,
    pub blkio_device_read_bps: Value,
    pub blkio_device_write_bps: Value,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Value,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Value,
    pub cpu_period: i64,
    pub cpu_quota: i64,
    pub cpu_realtime_period: i64,
    pub cpu_realtime_runtime: i64,
    pub cpuset_cpus: String,
    pub cpuset_mems: String,
    pub devices: Vec<Value>,
    pub disk_quota: i64,
    pub kernel_memory: i64,
    pub memory_reservation: i64,
    pub memory_swap: i64,
    pub memory_swappiness: i64,
    pub oom_kill_disable: bool,
    pub pids_limit: i64,
    pub ulimits: Vec<Ulimit>,
    pub cpu_count: i64,
    pub cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    pub iomaximum_iops: i64,
    #[serde(rename = "IOMaximumBandwidth")]
    pub iomaximum_bandwidth: i64,
    pub cgroup_conf: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub type_field: String,
    pub config: Value,
    pub path: String,
    pub tag: String,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub name: String,
    pub maximum_retry_count: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Idmappings {
    pub uid_map: Vec<String>,
    pub gid_map: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {
    pub name: String,
    pub soft: i64,
    pub hard: i64,
}
