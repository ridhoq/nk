extern crate crossbeam_deque;
extern crate priority_queue;
extern crate walkdir;

use std::io::Result as StdResult;
use std::fs;
use std::path::Path;
use std::thread;

use self::crossbeam_deque::{self as deque, Steal};
use self::walkdir::WalkDir;
use self::priority_queue::PriorityQueue;

pub fn nuke(dir_to_nuke: &str) -> StdResult<()> {
    let num_threads = 4;
    let (file_w, file_s) = deque::fifo();
    let (dir_w, dir_s) = deque::fifo();

    let mut file_pq = PriorityQueue::new();
    let mut dir_pq = PriorityQueue::new();

    for entry in WalkDir::new(dir_to_nuke).sort_by(|a, b| a.depth().cmp(&b.depth()).reverse()) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            dir_pq.push(entry.path().display().to_string(), entry.depth());
        }
        if entry.file_type().is_file() {
            file_pq.push(entry.path().display().to_string(), entry.depth());
        }
    }

    for (file, _) in file_pq.into_sorted_iter() {
        println!("{}", file);
        file_w.push(file);
    }

    for (dir, _) in dir_pq.into_sorted_iter() {
        println!("{}", dir);
        dir_w.push(dir);
    }

    let mut file_threads = vec![];
    for thread_num in 0..num_threads {
        let file_stealer = file_s.clone();
        file_threads.push(thread::Builder::new().name(format!("thread-{}", thread_num).to_string()).spawn(move || {
            while !file_stealer.is_empty() {
                let stolen = file_stealer.steal();
                match stolen {
                    Steal::Data(entry) => {
                        let path = Path::new(&entry);
                        println!("{} is deleting file: {}", thread::current().name().unwrap(), path.display());
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
        let _ = t.unwrap().join();
    }

    let mut dir_threads = vec![];
    for thread_num in 0..num_threads {
        let dir_stealer = dir_s.clone();
        dir_threads.push(thread::Builder::new().name(format!("thread-{}", thread_num).to_string()).spawn(move || {
            while !dir_stealer.is_empty() {
                let stolen = dir_stealer.steal();
                match stolen {
                    Steal::Data(entry) => {
                        let path = Path::new(&entry);
                        println!("{} is deleting dir: {}", thread::current().name().unwrap(), path.display());
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
        let _ = t.unwrap().join();
    }
    Ok(())
}

