extern crate crossbeam_deque;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use crossbeam_deque::{Worker};


fn print_path(path: &Path) {
    println!("Name: {}", path.display())
}

pub fn print_dir(dir: &Path) {
    visit_dirs(dir, print_entry);
}


pub fn add_dirs_to_deque(dir: &Path, deque: Worker<String>) -> Worker<String> {
    add_dirs_to_deque_recurse(dir, &deque);
    deque
}

pub fn add_dirs_to_deque_recurse(dir: &Path, deque: &Worker<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                add_dirs_to_deque_recurse(&path, deque);
                deque.push(path.display().to_string());
            } else {
                deque.push(path.display().to_string());
            }
        }
    }
    Ok(())
}

pub fn get_dirs_to_nuke(dir: &Path) -> Vec<String> {
    let mut vec = Vec::new();
    get_dirs_to_nuke_recurse(dir, &mut vec);
    vec
}

pub fn get_dirs_to_nuke_recurse(dir: &Path, vec: &mut Vec<String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                get_dirs_to_nuke_recurse(&path, vec);
            } else {
                vec.push(path.display().to_string());
            }
        }
    }
    Ok(())
}

pub fn visit_dirs(dir: &Path, cb: fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}


fn print_entry(entry: &DirEntry) {
    print_path(&entry.path());
}