use daemon;

fn main() {
    println!("hello from example");

    daemon::new();

    daemon::execute();

    daemon::user();

    daemon::home_dir();
}
