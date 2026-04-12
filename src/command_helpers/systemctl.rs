use std::{ffi::OsStr, process::Command};

pub fn run<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new("systemctl");
    cmd.arg("--user");
    cmd.args(args);

    let res = cmd.output().unwrap();
    if res.status.success() {
        return Ok(String::from_utf8_lossy(&res.stdout).into_owned());
    } else {
        return Err(String::from_utf8_lossy(&res.stderr).into_owned());
    }
}

pub fn restart(name: &str) {
    self::run(["restart", name]);
}

pub fn start(name: &str) {
    self::run(["start", name]);
}

pub fn stop(name: &str) {
    self::run(["stop", name]);
}

pub fn daemon_reload() {
    self::run(["daemon-reload"]);
}
