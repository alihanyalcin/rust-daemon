use rs_daemon;

fn main() {
    println!("hello from example");

    rs_daemon::new();

    rs_daemon::execute();

    rs_daemon::user();

    rs_daemon::home_dir();
}
