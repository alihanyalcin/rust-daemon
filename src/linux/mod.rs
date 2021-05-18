use crate::{path_exists, Daemon};

mod systemd;
mod systemv;

pub(crate) async fn new_daemon<S, I>(name: S, description: S, dependencies: I) -> Box<dyn Daemon>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    if path_exists("/run/systemd/system").await {
        Box::new(systemd::SystemD::new(name, description, dependencies))
    } else {
        Box::new(systemv::SystemV::new(name, description, dependencies))
    }
}
