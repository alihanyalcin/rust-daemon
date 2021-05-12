use crate::{command_output, command_status, Daemon};
use anyhow::{bail, Result};
use regex::Regex;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

pub(crate) struct SystemD {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    systemd_config: String,
}

#[allow(dead_code)]
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

    fn is_installed(&self) -> bool {
        Path::new(&self.service_path()).exists()
    }

    fn check_running(&self) -> Result<bool> {
        let output = command_output!("systemctl", "status", format!("{}.service", &self.name))?;

        if !output.status.success() {
            // TODO: if program is not running, status is 3.
            // https://www.freedesktop.org/software/systemd/man/systemctl.html#Exit%20status
            // bail!("service is stopped")
        }

        let is_active = Regex::new("Active: active")?;
        Ok(is_active.is_match(std::str::from_utf8(&output.stdout)?))
    }
}

impl Daemon for SystemD {
    fn get_template(&self) -> &str {
        &self.systemd_config
    }

    fn set_template(&mut self, new_config: &str) {
        self.systemd_config = new_config.to_string();
    }

    fn install(&self, args: Vec<&str>) -> Result<()> {
        crate::check_privileges()?;

        if self.is_installed() {
            bail!("service has already been installed")
        }

        let mut file = File::create(&self.service_path())?;

        let template = &self
            .systemd_config
            .replace("{Name}", &self.name)
            .replace("{Description}", &self.description)
            .replace("{Dependencies}", &self.dependencies.join(" "))
            .replace("{Path}", &crate::executable_path()?)
            .replace("{Args}", &args.join(" "));

        file.write(template.as_bytes())?;

        // TODO: success check ??
        command_status!("systemctl", "daemon-reload")?;

        command_status!("systemctl", "enable", &self.name)?;

        Ok(())
    }

    fn remove(&self) -> Result<()> {
        crate::check_privileges()?;

        if !self.is_installed() {
            bail!("service is not installed")
        }

        command_status!("systemctl", "disable", &self.name)?;

        remove_file(&self.service_path())?;

        Ok(())
    }

    fn start(&self) -> Result<()> {
        crate::check_privileges()?;

        if !self.is_installed() {
            bail!("service is not installed")
        }

        if self.check_running()? {
            bail!("service is already running")
        }

        command_status!("systemctl", "start", &self.name)?;

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        crate::check_privileges()?;

        if !self.is_installed() {
            bail!("service is not installed")
        }

        if !self.check_running()? {
            bail!("service has already been stopped")
        }

        command_status!("systemctl", "stop", &self.name)?;

        Ok(())
    }

    fn status(&self) -> Result<bool> {
        crate::check_privileges()?;

        if !self.is_installed() {
            bail!("service is not installed")
        }

        self.check_running()
    }
}
