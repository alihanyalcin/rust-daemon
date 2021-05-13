use crate::linux_systemd::SystemD;
use crate::linux_systemv::SystemV;
use crate::Daemon;
use std::path::Path;

pub(crate) fn new_daemon<S, I>(name: S, description: S, dependencies: I) -> Box<dyn Daemon>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    if Path::new("/run/systemd/system").exists() {
        Box::new(SystemD::new(name, description, dependencies))
    } else {
        Box::new(SystemV::new(name, description, dependencies))
    }
}
