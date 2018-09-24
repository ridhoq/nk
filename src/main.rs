mod nuke;
extern crate crossbeam_deque;

use std::path::Path;
use std::thread;
use std::fs;
use crossbeam_deque::{self as deque, Pop, Steal};

fn main() -> std::io::Result<()> {
    let dir = Path::new("./node_modules_nuke");
    nuke::traverse::print_dir(dir);
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
                        fs::remove_file(value.as_str());
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
