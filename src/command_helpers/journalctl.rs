use std::{ffi::OsStr, process::Command};

fn run<I, S>(args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new("journalctl");
    cmd.arg("--user");
    cmd.args(args);

    let res = cmd.output().unwrap();
    if res.status.success() {
        return Ok(String::from_utf8_lossy(&res.stdout).into_owned());
    } else {
        return Err(String::from_utf8_lossy(&res.stderr).into_owned());
    }
}

pub fn stream_logs(unit: &str) {}

pub fn unit_logs(unit: &str) -> Result<String, String> {
    run(["-u", unit, "--no-pager"])
}
