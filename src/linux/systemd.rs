use crate::{command_output, command_status, path_exists, Daemon, DaemonError};
use anyhow::{bail, Result};
use async_trait::async_trait;
use log::trace;
use regex::Regex;
use tokio::{
    fs::{remove_file, File},
    io::AsyncWriteExt,
};

pub(crate) struct SystemD {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    systemd_config: String,
}

impl SystemD {
    pub fn new<S, I>(name: S, description: S, dependencies: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            dependencies: dependencies.into_iter().map(Into::into).collect(),
            systemd_config: r#"[Unit]
Description={Description}
Requires={Dependencies}
After={Dependencies}

[Service]
PIDFile=/var/run/{Name}.pid
ExecStartPre=/bin/rm -f /var/run/{Name}.pid
ExecStart={Path} {Args}
Restart=on-failure

[Install]
WantedBy=multi-user.target
"#
            .into(),
        }
    }

    fn service_path(&self) -> String {
        format!("/etc/systemd/system/{}.service", &self.name)
    }

    async fn is_installed(&self) -> bool {
        path_exists(&self.service_path()).await
    }

    async fn is_running(&self) -> Result<bool> {
        let output = command_output!("systemctl", "status", format!("{}.service", &self.name))?;

        // https://www.freedesktop.org/software/systemd/man/systemctl.html#Exit%20status
        let code = output.status.code();
        if Some(0) != code {
            if Some(3) == code {
                trace!("program is not running");
                return Ok(false);
            }
            trace!("program is dead or status is unknown");

            bail!(DaemonError::Stopped)
        }

        let is_active = Regex::new("Active: active")?;
        Ok(is_active.is_match(std::str::from_utf8(&output.stdout)?))
    }
}

#[async_trait]
impl Daemon for SystemD {
    fn get_config(&self) -> &str {
        &self.systemd_config
    }

    fn set_config(&mut self, new_config: &str) {
        self.systemd_config = new_config.to_string();
    }

    async fn install(&self, args: Vec<&str>) -> Result<()> {
        trace!("service is installing");

        crate::check_privileges().await?;

        if self.is_installed().await {
            bail!(DaemonError::AlreadyInstalled)
        }

        let template = &self
            .systemd_config
            .replace("{Name}", &self.name)
            .replace("{Description}", &self.description)
            .replace("{Dependencies}", &self.dependencies.join(" "))
            .replace("{Path}", &crate::executable_path()?)
            .replace("{Args}", &args.join(" "));

        let mut file = File::create(&self.service_path()).await?;

        file.write_all(template.as_bytes()).await?;

        let status = command_status!("systemctl", "daemon-reload")?;
        if !status.success() {
            bail!("daemon-reload failed")
        }

        let status = command_status!("systemctl", "enable", &self.name)?;
        if !status.success() {
            bail!("enable failed")
        }

        Ok(())
    }

    async fn remove(&self) -> Result<()> {
        trace!("service is removing");

        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!(DaemonError::NotInstalled)
        }

        command_status!("systemctl", "disable", &self.name)?;

        remove_file(&self.service_path()).await?;

        Ok(())
    }

    async fn start(&self) -> Result<()> {
        trace!("service is starting");

        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!(DaemonError::NotInstalled)
        }

        if self.is_running().await? {
            bail!(DaemonError::AlreadyRunning)
        }

        let status = command_status!("systemctl", "start", &self.name)?;
        if !status.success() {
            bail!(DaemonError::StartFailed)
        }

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        trace!("service is stopping");

        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!(DaemonError::NotInstalled)
        }

        if !self.is_running().await? {
            bail!(DaemonError::AlreadyStopped)
        }

        let status = command_status!("systemctl", "stop", &self.name)?;
        if !status.success() {
            bail!(DaemonError::StopFailed)
        }

        Ok(())
    }

    async fn status(&self) -> Result<bool> {
        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!(DaemonError::NotInstalled)
        }

        Ok(self.is_running().await?)
    }
}
