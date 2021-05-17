use crate::{linux_systemd::SystemD, linux_systemv::SystemV, path_exists, Daemon};

pub(crate) async fn new_daemon<S, I>(name: S, description: S, dependencies: I) -> Box<dyn Daemon>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    if path_exists("/run/systemd/system").await {
        Box::new(SystemD::new(name, description, dependencies))
    } else {
        Box::new(SystemV::new(name, description, dependencies))
    }
}
