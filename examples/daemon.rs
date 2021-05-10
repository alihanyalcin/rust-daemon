use daemon::Daemon;

fn main() {
    let daemon = match daemon::new("name", "description", vec!["d1", "d2"]) {
        Ok(daemon) => daemon,
        Err(err) => panic!("{}", err),
    };

    println!("{}", daemon.get_template());

    match daemon.install(vec!["args1", "args2"]) {
        Ok(()) => println!("ok"),
        Err(err) => println!("{}", err),
    }

    //daemon.set_template("new_config");

    //println!("updated: \n{}", daemon.get_template());

    //match daemon::check_privileges() {
    //Ok(_) => println!("root"),
    //Err(err) => println!("{}", err),
    //}

    //match daemon::executable_path() {
    //Ok(path) => println!("path: {}", path),
    //Err(err) => println!("{}", err),
    //}

    // daemon::execute();

    // daemon::user();

    // daemon::home_dir();
}
