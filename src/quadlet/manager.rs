use std::path::PathBuf;

use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    sync::mpsc::UnboundedSender,
};

use crate::{
    command_helpers::{journalctl, systemctl},
    quadlet::types::{QuadletBasicInfo, QuadletDetailedInfo, QuadletKind},
    tui::event::{AppEvent, Event},
};

pub fn list() -> std::io::Result<Vec<QuadletBasicInfo>> {
    let cmd = std::process::Command::new("podman")
        .args(["quadlet", "list", "--format", "json"])
        .output()?;
    let res = serde_json::from_slice::<Vec<QuadletBasicInfo>>(&cmd.stdout)?
        .into_iter()
        .map(|q| {
            let parts = q.file_name.rsplit_once('.').unwrap();
            let name = parts.0.to_string();
            let kind = QuadletKind::from(parts.1);
            QuadletBasicInfo { name, kind, ..q }
        })
        .collect();

    Ok(res)
}

pub fn inspect(q: &QuadletBasicInfo) -> std::io::Result<QuadletDetailedInfo> {
    QuadletDetailedInfo::try_from(q)
}

pub fn start(name: &str) {
    let res = systemctl::run(["stop", name]);
}

pub fn stop(name: &str) {
    let res = systemctl::run(["stop", name]);
}

pub async fn stream_logs(unit_name: String, tx: UnboundedSender<Event>) {
    let mut child = tokio::process::Command::new("journalctl")
        .arg("--user")
        .arg("-fu")
        .arg(&unit_name)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut out_reader = BufReader::new(stdout).lines();
    let mut err_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            Ok(Some(line)) = out_reader.next_line() => {
                tx.send(Event::App(AppEvent::AppendLog(unit_name.clone(), line))).ok();
            }
            Ok(Some(line)) = err_reader.next_line() => {
                tx.send(Event::App(AppEvent::AppendLog(unit_name.clone(), line))).ok();
            }
            else => break,
        }
    }

    let _ = child.wait().await;
}

pub async fn restart(name: String, tx: UnboundedSender<Event>) {
    let mut child = tokio::process::Command::new("systemctl")
        .arg("--user")
        .arg("restart")
        .arg("-v")
        .arg(&name)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let mut out_reader = BufReader::new(stdout).lines();
    let mut err_reader = BufReader::new(stderr).lines();

    loop {
        tokio::select! {
            // Ok(Some(line)) = out_reader.next_line() => {
            //     tx.send(Message::PopupLog(line)).ok();
            // }
            // Ok(Some(line)) = err_reader.next_line() => {
            //     tx.send(Message::PopupLog(line)).ok();
            // }
            else => break,
        }
    }

    let _ = child.wait().await;
}

pub fn journal(name: &str) -> Result<String, String> {
    // TODO: if the unit isnt found, this returns OK("-- No Entries --"), that should be an error
    journalctl::unit_logs(name)
}

pub fn edit(path: PathBuf) {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());

    let mut parts = editor.split_whitespace();
    let cmd = parts.next().unwrap();
    let args: Vec<_> = parts.collect();

    let _ = std::process::Command::new(cmd)
        .args(&args)
        .arg(&path)
        .status();
}
