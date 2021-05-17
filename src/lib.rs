use anyhow::{bail, Result};
use async_trait::async_trait;
use std::env::{consts::OS, current_exe};
use thiserror::Error;

mod linux;
mod linux_systemd;
mod linux_systemv;
mod macros;

#[async_trait]
pub trait Daemon {
    fn get_config(&self) -> &str;
    fn set_config(&mut self, new_config: &str);
    async fn install(&self, args: Vec<&str>) -> Result<()>;
    async fn remove(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn status(&self) -> Result<bool>;
    // fn run(e: impl Executable) -> Result<&str>;
}

#[derive(Error, Debug)]
enum DaemonError {
    #[error("operating system is not supported")]
    OSNotSupported,

    #[error("unsupported system")]
    UnsupportedSystem,

    #[error("you must have root privileges")]
    RootPrivileges,

    #[error("cannot get current running executable")]
    ExecutablePath,

    #[error("service is stopped")]
    Stopped,

    #[error("service has already been stopped")]
    AlreadyStopped,

    #[error("failed to stop service")]
    StopFailed,

    #[error("service has already been installed")]
    AlreadyInstalled,

    #[error("service is not installed")]
    NotInstalled,

    #[error("service is already running")]
    AlreadyRunning,

    #[error("failed to start service")]
    StartFailed,
}

//pub enum Status {
//ACTIVE,
//INACTIVE,
//}

//trait Executable {
//    fn start();
//    fn stop();
//    fn run();
//}

pub async fn new<S, I>(name: S, description: S, dependencies: I) -> Result<Box<dyn Daemon>>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    match OS {
        "linux" => Ok(linux::new_daemon(name, description, dependencies).await),
        "macos" => Ok(linux::new_daemon(name, description, dependencies).await),
        _ => bail!(DaemonError::OSNotSupported),
    }
}

async fn check_privileges() -> Result<()> {
    let output = command_output!("id", "-g")?;

    if !output.status.success() {
        bail!(DaemonError::UnsupportedSystem)
    }

    match String::from_utf8(output.stdout)?.trim().parse::<u32>()? {
        0 => Ok(()),
        _ => bail!(DaemonError::RootPrivileges),
    }
}

fn executable_path() -> Result<String> {
    match current_exe()?.into_os_string().into_string() {
        Ok(exe) => Ok(exe),
        Err(_) => bail!(DaemonError::ExecutablePath),
    }
}

async fn path_exists(path: &str) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}
