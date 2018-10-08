extern crate rand;

use self::rand::distributions::Alphanumeric;
use self::rand::{thread_rng, Rng};
use std::fs::{self, File};
use std::io::prelude::*;
use std::iter;
use std::path::PathBuf;

pub fn generate_rand_str(size: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(size)
        .collect()
}

pub fn write_file(size: usize, path: &PathBuf) {
    let mut f = File::create(path).expect("unable to create file");
    f.write_all(generate_rand_str(size).as_bytes());
    f.sync_all();
}

pub fn write_dir(path: &PathBuf) {
    fs::create_dir_all(path);
}
