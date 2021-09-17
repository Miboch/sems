use std::str;
// use std::env;
use std::process::Command;

fn main() {
    let stdout = Command::new("git").arg("tag").output().unwrap().stdout;
    let tags = str::from_utf8(stdout.as_slice()).unwrap();
    let tag_vector = tags.split("\n");
    let mut sem_vers: Vec<SemVer> = Vec::new();
    for s in tag_vector {
        if s.contains(".") {
            sem_vers.push(SemVer::new(s));
        }
    }

    sem_vers.sort_by(|a,b| a.patch.cmp(&b.patch));
    for s in sem_vers {
        println!("{:?}", s)
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct SemVer {
    major: i32,
    minor: i32,
    patch: i32
}

impl SemVer {
    fn new(version: &str) -> SemVer {
        let split: Vec<&str> = version.split(".").collect();
        SemVer {
            major: split[0].parse().unwrap(),
            minor: split[1].parse().unwrap(),
            patch: split[2].parse().unwrap(),
        }
    }
}