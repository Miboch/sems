use std::str;
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
    sem_vers.sort_by(|a, b| b.patch.cmp(&a.patch));
    sem_vers.sort_by(|a, b| b.minor.cmp(&a.minor));
    sem_vers.sort_by(|a, b| b.major.cmp(&a.major));
    println!(
        "{}.{}.{}",
        sem_vers[0].major, sem_vers[0].minor, sem_vers[0].patch
    );
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct SemVer {
    major: i32,
    minor: i32,
    patch: i32,
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
