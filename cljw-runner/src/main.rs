use std::io;

use lib_runner;
fn main() {
    lib_runner::run(&[".clj",".cljw"]);
    let mut buf = String::new();
    println!("\npress enter to leave.");
    io::stdin()
        .read_line(&mut buf)
        .expect("read line failed");
}