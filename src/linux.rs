use crate::linux_systemd::SystemD;
use crate::Daemon;
use anyhow::Result;
use std::path::Path;

// TODO: no need to return Result
pub(crate) fn new_daemon<S, I>(name: S, description: S, dependencies: I) -> Result<impl Daemon>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    if Path::new("/run/systemd/system").exists() {
        return Ok(SystemD::new(name, description, dependencies));
    }

    Ok(SystemD::new(name, description, dependencies))
}
