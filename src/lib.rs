use anyhow::{bail, Result};
use std::process::Command;
use users::{get_current_uid, get_user_by_uid};

mod linux;
mod linux_systemd;

pub trait Daemon {
    fn get_template(&self) -> &str;
    fn set_template(&mut self, new_config: &str);
    // TODO: should take ards
    // fn install() -> Result<&str>;
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

pub fn new<'a>(name: &'a str, description: &str, dependencies: Vec<String>) -> Result<impl Daemon> {
    match std::env::consts::OS {
        "linux" => linux::new_daemon(name.to_string(), description.to_string(), dependencies),
        "macos" => linux::new_daemon(name.to_string(), description.to_string(), dependencies),
        _ => bail!("operating system is not supported"),
    }
}

pub fn execute() {
    //let output = Command::new("ls")
    //.arg("-c")
    //.output()
    //.expect("failed to execute process");

    //let hello = output.stdout;

    //println!("{:?}", std::str::from_utf8(&hello));

    //let mut list_dir = Command::new("ls");

    // Execute `ls` in the current directory of the program.
    //list_dir.status().expect("process failed to execute");

    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("Command executed");
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
