extern crate crossbeam_deque;
extern crate walkdir;

use std::collections::HashMap;
use std::fs;
use std::io::Result as StdResult;
use std::path::Path;
use std::thread;

use self::crossbeam_deque::{self as deque, Steal, Stealer, Worker};
use self::walkdir::WalkDir;

struct NukeDeque {
    worker: Worker<String>,
    stealer: Stealer<String>,
}

pub fn nuke(dir_to_nuke: &str) -> StdResult<()> {
    let num_threads = 4;

    let mut depth_to_deque: HashMap<usize, NukeDeque> = HashMap::new();
    let mut max_depth: usize = 0;

    for entry in WalkDir::new(dir_to_nuke).sort_by(|a, b| a.depth().cmp(&b.depth()).reverse()) {
        let entry = entry.unwrap();
        let depth = entry.depth();
        if depth > max_depth {
            max_depth = depth;
        }

        depth_to_deque
            .entry(depth)
            .and_modify(|e| {
                e.worker.push(entry.path().display().to_string());
            }).or_insert_with(|| {
                let (worker, stealer) = deque::fifo();
                worker.push(entry.path().display().to_string());
                NukeDeque { worker, stealer }
            });
    }

    for d in (0..max_depth + 1).rev() {
        let mut threads = vec![];
        for thread_num in 0..num_threads {
            let mut nd = depth_to_deque.get_mut(&d).unwrap();
            let stealer = &nd.stealer;
            let thread_stealer = stealer.clone();
            threads.push(
                thread::Builder::new()
                    .name(format!("thread-{}", thread_num).to_string())
                    .spawn(move || {
                        while !thread_stealer.is_empty() {
                            let stolen = thread_stealer.steal();
                            match stolen {
                                Steal::Data(entry) => {
                                    let path = Path::new(&entry);
                                    println!(
                                        "{} is deleting: {}",
                                        thread::current().name().unwrap(),
                                        path.display()
                                    );
                                    if path.is_dir() {
                                        fs::remove_dir(path).expect("Failed to remove a dir");
                                    }
                                    if path.is_file() {
                                        fs::remove_file(path).expect("Failed to remove a file");
                                    }
                                }
                                Steal::Empty => {}
                                Steal::Retry => {}
                            }
                        }
                    }),
            )
        }

        for t in threads {
            let _ = t.unwrap().join();
        }
    }
    Ok(())
}
