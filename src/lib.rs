extern crate crossbeam_deque;
extern crate num_cpus;
extern crate walkdir;

use std::collections::HashMap;
use std::fs;
use std::io::Result as StdResult;
use std::path::{Path, PathBuf};
use std::thread;

use self::crossbeam_deque::{self as deque, Steal, Stealer, Worker};
use self::walkdir::WalkDir;

struct NukeDeque {
    worker: Worker<PathBuf>,
    stealer: Stealer<PathBuf>,
}

pub fn nuke(dir_to_nuke: &PathBuf) -> StdResult<()> {
    if !dir_to_nuke.exists() {
        return Ok(());
    }

    let num_threads = num_cpus::get();
    let mut depth_to_deque: HashMap<usize, NukeDeque> = HashMap::new();
    let mut max_depth: usize = 0;

    for entry in WalkDir::new(dir_to_nuke) {
        let entry = entry.unwrap();
        let depth = entry.depth();
        if depth > max_depth {
            max_depth = depth;
        }

        depth_to_deque
            .entry(depth)
            .and_modify(|e| {
                e.worker.push(PathBuf::from(entry.path()));
            }).or_insert_with(|| {
                let (worker, stealer) = deque::fifo();
                worker.push(PathBuf::from(entry.path()));
                NukeDeque { worker, stealer }
            });
    }

    for d in (0..max_depth + 1).rev() {
        let mut threads = vec![];
        let mut nd = depth_to_deque.get_mut(&d).unwrap();
        if !&nd.stealer.is_empty() {
            for thread_num in 0..num_threads {
                let stealer = &nd.stealer;
                let thread_stealer = stealer.clone();
                threads.push(
                    thread::Builder::new()
                        .name(format!("thread-{}", thread_num).to_string())
                        .spawn(move || {
                            while !thread_stealer.is_empty() {
                                let stolen = thread_stealer.steal();
                                match stolen {
                                    Steal::Data(path_buf) => {
                                        let path = path_buf.as_path();
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
    }
    Ok(())
}
