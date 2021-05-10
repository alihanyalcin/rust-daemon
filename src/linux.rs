use crate::linux_systemd::SystemD;
use crate::Daemon;
use anyhow::Result;
use std::path::Path;

// TODO: no need to return Result
pub(crate) fn new_daemon(
    name: String,
    description: String,
    dependencies: Vec<&str>,
) -> Result<impl Daemon> {
    if Path::new("/run/systemd/system").exists() {
        return Ok(SystemD::new(name, description, dependencies));
    }

    Ok(SystemD::new(name, description, dependencies))
}
