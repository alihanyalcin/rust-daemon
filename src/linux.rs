use crate::linux_systemd::SystemD;
use crate::Daemon;
use anyhow::Result;

pub(crate) fn new_daemon(
    name: String,
    description: String,
    dependencies: Vec<String>,
) -> Result<impl Daemon> {
    Ok(SystemD::new(name, description, dependencies))
}
