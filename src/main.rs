use walkdir::WalkDir;
use std::env;
use std::io::{self};
use std::io::Write;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use sha256::{digest, digest_bytes};


fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();

    let directory = &args[1];
    search(directory.to_string());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn search(dir: String) {
    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => files(entry.path().display().to_string()),
            Err(err) => {
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {
                            continue;
                        }
                        io::ErrorKind::PermissionDenied => {
                            continue;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn files(file: String) {
    let fileu8 = file.as_bytes();
    let hash = digest_bytes(&fileu8);
    let mut files = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("FileOut.txt")
        .unwrap();
    writeln!(files, "{}", hash);
    let args: Vec<String> = env::args().collect();
    let SHA = &args[2];
    if SHA == &hash {
        println!{"Hash found {} for file {}", hash, file};
    }
}

fn compare() {

}
