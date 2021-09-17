use std::str;
// use std::env;
use std::{process::Command};

fn main() {

    let stdout = Command::new("git")
    .arg("tag")
    .output()
    .unwrap().stdout;
    let tags = str::from_utf8(stdout.as_slice()).unwrap();
    // let args: Vec<String> = env::args().collect();
    println!("{:?}", tags);
}
