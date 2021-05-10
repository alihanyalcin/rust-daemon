use crate::Daemon;
use anyhow::{bail, Result};
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub(crate) struct SystemD {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    systemd_config: String,
}

#[allow(dead_code)]
impl SystemD {
    pub fn new(name: String, description: String, dependencies: Vec<&str>) -> Self {
        Self {
            name,
            description,
            dependencies: dependencies
                .iter()
                .map(|&s| s.to_string())
                .collect::<Vec<String>>(),
            systemd_config: r#"
[Unit]
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
            .to_string(),
        }
    }

    fn service_path(&self) -> String {
        format!("/etc/systemd/system/{}/.service", &self.name)
    }

    fn is_installed(&self) -> bool {
        Path::new(&self.service_path()).exists()
    }

    fn check_running(&self) -> Result<bool> {
        let output = Command::new("systemctl")
            .arg("status")
            .arg(format!("{}.service", &self.name))
            .output()?;

        if !output.status.success() {
            bail!("service is stopped")
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

        Command::new("systemctl").arg("daemon-reload").status()?;

        Command::new("systemctl")
            .arg("enable")
            .arg(&self.name)
            .status()?;

        Ok(())
    }
}
