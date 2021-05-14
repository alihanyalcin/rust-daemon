use crate::{command_output, command_status, path_exist, Daemon};
use anyhow::{bail, Result};
use async_trait::async_trait;
use log::trace;
use regex::Regex;
use std::os::unix::fs::PermissionsExt;
use tokio::fs::{remove_file, symlink, File};
use tokio::io::AsyncWriteExt;

pub(crate) struct SystemV {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    systemv_config: String,
}

impl SystemV {
    pub fn new<S, I>(name: S, description: S, dependencies: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            dependencies: dependencies.into_iter().map(Into::into).collect(),
            systemv_config: r#"#! /bin/sh
#
#       /etc/rc.d/init.d/{Name}
#
#       Starts {Name} as a daemon
#
# chkconfig: 2345 87 17
# description: Starts and stops a single {Name} instance on this system

### BEGIN INIT INFO
# Provides: {Name} 
# Required-Start: $network $named
# Required-Stop: $network $named
# Default-Start: 2 3 4 5
# Default-Stop: 0 1 6
# Short-Description: This service manages the {Description}.
# Description: {Description}
### END INIT INFO

#
# Source function library.
#
if [ -f /etc/rc.d/init.d/functions ]; then
    . /etc/rc.d/init.d/functions
fi

exec="{Path}"
servname="{Description}"

proc="{Name}"
pidfile="/var/run/$proc.pid"
lockfile="/var/lock/subsys/$proc"
stdoutlog="/var/log/$proc.log"
stderrlog="/var/log/$proc.err"

[ -d $(dirname $lockfile) ] || mkdir -p $(dirname $lockfile)

[ -e /etc/sysconfig/$proc ] && . /etc/sysconfig/$proc

start() {
    [ -x $exec ] || exit 5

    if [ -f $pidfile ]; then
        if ! [ -d "/proc/$(cat $pidfile)" ]; then
            rm $pidfile
            if [ -f $lockfile ]; then
                rm $lockfile
            fi
        fi
    fi

    if ! [ -f $pidfile ]; then
        printf "Starting $servname:\t"
        echo "$(date)" >> $stdoutlog
        $exec {Args} >> $stdoutlog 2>> $stderrlog &
        echo $! > $pidfile
        touch $lockfile
        success
        echo
    else
        # failure
        echo
        printf "$pidfile still exists...\n"
        exit 7
    fi
}

stop() {
    echo -n $"Stopping $servname: "
    killproc -p $pidfile $proc
    retval=$?
    echo
    [ $retval -eq 0 ] && rm -f $lockfile
    return $retval
}

restart() {
    stop
    start
}

rh_status() {
    status -p $pidfile $proc
}

rh_status_q() {
    rh_status >/dev/null 2>&1
}

case "$1" in
    start)
        rh_status_q && exit 0
        $1
        ;;
    stop)
        rh_status_q || exit 0
        $1
        ;;
    restart)
        $1
        ;;
    status)
        rh_status
        ;;
    *)
        echo $"Usage: $0 {start|stop|status|restart}"
        exit 2
esac

exit $?
"#
            .into(),
        }
    }

    fn service_path(&self) -> String {
        format!("/etc/init.d/{}", &self.name)
    }

    async fn is_installed(&self) -> bool {
        path_exist!(&self.service_path())
    }

    async fn is_running(&self) -> Result<bool> {
        let output = command_output!("service", &self.name, "status")?;

        let code = output.status.code();
        if Some(0) != code {
            if Some(3) == code {
                trace!("program is not running");
                return Ok(false);
            }
            trace!("program is dead or status is unknown");

            bail!("service is stopped")
        }

        let is_active = Regex::new(&self.name)?;
        Ok(is_active.is_match(std::str::from_utf8(&output.stdout)?))
    }
}

#[async_trait]
impl Daemon for SystemV {
    fn get_template(&self) -> &str {
        &self.systemv_config
    }

    fn set_template(&mut self, new_config: &str) {
        self.systemv_config = new_config.to_string();
    }

    async fn install(&self, args: Vec<&str>) -> Result<()> {
        trace!("service is installing");

        crate::check_privileges().await?;

        if self.is_installed().await {
            bail!("service has already been installed")
        }

        let template = &self
            .systemv_config
            .replace("{Name}", &self.name)
            .replace("{Description}", &self.description)
            .replace("{Path}", &crate::executable_path()?)
            .replace("{Args}", &args.join(" "));

        let service_path = &self.service_path();
        let mut file = File::create(service_path).await?;

        file.write_all(template.as_bytes()).await?;

        let metadata = file.metadata().await?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);

        for i in vec!["2", "3", "4", "5"] {
            symlink(service_path, format!("/etc/rc{}.d/S87{}", i, &self.name)).await?;
        }

        for i in vec!["0", "1", "6"] {
            symlink(service_path, format!("/etc/rc{}.d/K17{}", i, &self.name)).await?;
        }

        Ok(())
    }

    async fn remove(&self) -> Result<()> {
        trace!("service is removing");

        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!("service is not installed")
        }

        remove_file(&self.service_path()).await?;

        for i in vec!["2", "3", "4", "5"] {
            remove_file(format!("/etc/rc{}.d/S87{}", i, &self.name)).await?;
        }

        for i in vec!["0", "1", "6"] {
            remove_file(format!("/etc/rc{}.d/K17{}", i, &self.name)).await?;
        }

        Ok(())
    }

    async fn start(&self) -> Result<()> {
        trace!("service is starting");

        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!("service is not installed")
        }

        if self.is_running().await? {
            bail!("service is already running")
        }

        let status = command_status!("service", &self.name, "start")?;
        if !status.success() {
            bail!("failed to start service")
        }

        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        trace!("service is stopping");

        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!("service is not installed")
        }

        if !self.is_running().await? {
            bail!("service has already been stopped")
        }

        command_status!("service", &self.name, "stop")?;

        Ok(())
    }

    async fn status(&self) -> Result<bool> {
        crate::check_privileges().await?;

        if !self.is_installed().await {
            bail!("service is not installed")
        }

        Ok(self.is_running().await?)
    }
}
