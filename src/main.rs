mod nuke;
extern crate crossbeam_deque;
extern crate walkdir;

use std::path::Path;
use std::thread;
use std::fs;

use crossbeam_deque::{self as deque, Pop, Steal};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let dir = Path::new("./node_modules_nuke");
    let num_threads = 4;
    let (file_w, file_s) = deque::fifo();
    let (dir_w, dir_s) = deque::fifo();

    for entry in WalkDir::new(".\\node_modules_nuke") {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            dir_w.push(entry.path().display().to_string());
        }
        if entry.file_type().is_file() {
            file_w.push(entry.path().display().to_string());
        }
        println!("{}", entry.path().display());
    }

    let mut file_threads = vec![];
    for _ in 0..num_threads {
        let file_stealer = file_s.clone();
        file_threads.push(thread::spawn(move || {
            while !file_stealer.is_empty() {
                let stolen = file_stealer.steal();
                match stolen {
                    Steal::Data(entry) => {
                        let path = Path::new(&entry);
                        println!("deleting file: {}", path.display());
                        fs::remove_file(path).expect("Failed to remove a file");
                    },
                    Steal::Empty => {},
                    Steal::Retry => {}
                }
            }
        }))
    }
    for t in file_threads {
        // Wait for the thread to finish. Returns a result.
        let _ = t.join();
    }

    let mut dir_threads = vec![];
    for _ in 0..num_threads {
        let dir_stealer = dir_s.clone();
        dir_threads.push(thread::spawn(move || {
            while !dir_stealer.is_empty() {
                let stolen = dir_stealer.steal();
                match stolen {
                    Steal::Data(entry) => {
                        let path = Path::new(&entry);
                        println!("deleting dir: {}", path.display());
                        fs::remove_dir(path).expect("Failed to remove a dir");
                    },
                    Steal::Empty => {},
                    Steal::Retry => {}
                }
            }
        }))
    }
    for t in dir_threads {
        // Wait for the thread to finish. Returns a result.
        let _ = t.join();
    }
    Ok(())
}

fn remove_dir_or_file(path_str: &str) {
    let path = Path::new(path_str);
    if path.is_dir() {
        println!("deleting dir: {}", path_str);
        fs::remove_dir(path).expect(&format!("Failed to remove a dir: {}", path_str));
    }
    if path.is_file() {
        println!("deleting file: {}", path_str);
        fs::remove_file(path).expect("Failed to remove a file");
    }
}
