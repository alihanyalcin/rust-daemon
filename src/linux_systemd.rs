use crate::Daemon;

pub(crate) struct SystemD {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    systemd_config: String,
}

impl SystemD {
    pub fn new(name: String, description: String, dependencies: Vec<String>) -> Self {
        Self {
            name,
            description,
            dependencies,
            systemd_config: r#"
[Unit]
Description={Description}
Requires={Dependencies}
After={Dependencies}

[Service]
PIDFile=/var/run/{Name}.pid
ExecStartPre=/bin/rm -f /var/run/{Name}.pid
ExecStart={{.Path}} {Args}
Restart=on-failure

[Install]
WantedBy=multi-user.target
            "#
            .to_string(),
        }
    }
}

impl Daemon for SystemD {
    fn get_template(&self) -> &str {
        &self.systemd_config
    }

    fn set_template(&mut self, new_config: &str) {
        self.systemd_config = new_config.to_string();
    }
}
