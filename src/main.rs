use std::str;
// use std::env;
use std::process::Command;

fn main() {
    let stdout = Command::new("git").arg("tag").output().unwrap().stdout;
    let tags = str::from_utf8(stdout.as_slice()).unwrap();
    let mut tag_vector = tags.split("\n");
    for s in tag_vector {
        println!("{:?}", s);
    }
}
