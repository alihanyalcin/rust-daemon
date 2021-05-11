use anyhow::{bail, Result};
use std::env::{consts::OS, current_exe};
use users::{get_current_uid, get_user_by_uid};

mod linux;
mod linux_systemd;
mod macros;

// TODO: generics ?? type ??
pub trait Daemon {
    fn get_template(&self) -> &str;
    fn set_template(&mut self, new_config: &str);
    fn install(&self, args: Vec<&str>) -> Result<()>;
    // fn remove() -> Result<&str>;
    // fn start() -> Result<&str>;
    // fn stop() -> Result<&str>;
    // fn status() -> Result<&str>;
    // fn run(e: impl Executable) -> Result<&str>;
}

//trait Executable {
//    fn start();
//    fn stop();
//    fn run();
//}

pub fn new<S, I>(name: S, description: S, dependencies: I) -> Result<impl Daemon>
where
    S: Into<String>,
    I: IntoIterator<Item = S>,
{
    match OS {
        "linux" => linux::new_daemon(name, description, dependencies),
        "macos" => linux::new_daemon(name, description, dependencies),
        _ => bail!("operating system is not supported"),
    }
}

pub(crate) fn check_privileges() -> Result<()> {
    let output = command_output!("id", "-g")?;

    if !output.status.success() {
        bail!("unsupported system")
    }

    match String::from_utf8(output.stdout)?.trim().parse::<u32>()? {
        0 => Ok(()),
        _ => bail!("you must have root privileges"),
    }
}

pub(crate) fn executable_path() -> Result<String> {
    match current_exe()?.into_os_string().into_string() {
        Ok(exe) => Ok(exe),
        Err(_) => bail!("cannot get current running executable"),
    }
}

pub fn user() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("Hello, {:?}!", user.name());
}

pub fn home_dir() {
    let home = dirs::home_dir();
    println!("home_dir: {:?}", home)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
