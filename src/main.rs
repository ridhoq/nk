mod nuke;
extern crate crossbeam_deque;

use std::path::Path;
use std::thread;
use std::fs;
use crossbeam_deque::{self as deque, Pop, Steal};

fn main() -> std::io::Result<()> {
    let dir = Path::new("./node_modules_nuke");
    // nuke::traverse::print_dir(dir);
    let (empty_worker, s) = deque::fifo();
    nuke::traverse::add_dirs_to_deque(dir, empty_worker);
    let mut children = vec![];
    for _ in 0..3 {
        let steal = s.clone();
        children.push(thread::spawn(move || {
            while !steal.is_empty() {
                let popped = steal.steal();
                match popped {
                    Steal::Data(value) => {
                        remove_dir_or_file(value.as_str());
                    },
                    Steal::Empty => {},
                    Steal::Retry => {}
                }
            }
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
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
