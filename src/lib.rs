use std::process::Command;
use users::{get_current_uid, get_user_by_uid};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn new() {
    match std::env::consts::OS {
        "linux" => println!("linux"),
        "macos" => println!("macos"),
        _ => println!("not supported"),
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
