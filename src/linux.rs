use crate::linux_systemd::SystemD;
use crate::linux_systemv::SystemV;
use crate::path_exist;
use crate::Daemon;

pub(crate) async fn new_daemon<S, I>(name: S, description: S, dependencies: I) -> Box<dyn Daemon>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    if path_exist!("/run/systemd/system") {
        Box::new(SystemD::new(name, description, dependencies))
    } else {
        Box::new(SystemV::new(name, description, dependencies))
    }
}
