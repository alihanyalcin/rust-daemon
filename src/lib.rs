//! # Daemon
//!
//! `daemon` is a async Rust version of Go [daemon](https://github.com/takama/daemon) package

use anyhow::{bail, Result};
use async_trait::async_trait;
use std::env::{consts::OS, current_exe};
use thiserror::Error;

mod linux;
mod macros;

#[async_trait]
pub trait Daemon {
    /// gets service config
    fn get_config(&self) -> &str;
    /// sets service config
    fn set_config(&mut self, new_config: &str);
    /// Install the service into the system
    /// ## Example
    /// ```rust
    /// match daemon.install(daemon::no_args!()).await {
    ///   Ok(()) => {
    ///        info!("installed");
    ///    }
    ///    Err(err) => bail!("install error: {}", err),
    /// };
    /// ```
    async fn install(&self, args: Vec<&str>) -> Result<()>;
    /// Remove the service and all corresponding files from the system
    async fn remove(&self) -> Result<()>;
    /// Start the service
    async fn start(&self) -> Result<()>;
    /// Stop the service
    async fn stop(&self) -> Result<()>;
    /// Check the service status
    async fn status(&self) -> Result<bool>;
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

pub async fn new<S, I>(name: S, description: S, dependencies: I) -> Result<Box<dyn Daemon>>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    match OS {
        "linux" => Ok(linux::new_daemon(name, description, dependencies).await),
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
