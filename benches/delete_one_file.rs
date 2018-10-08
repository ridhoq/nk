#[macro_use]
extern crate criterion;
extern crate nuke;

use criterion::Criterion;
use std::path::PathBuf;
mod utils;

const FILE_SIZE: usize = 500000;

fn delete_one_file_nuke(c: &mut Criterion) {
    let mut path = PathBuf::from("../delete_one_file/nuke");
    utils::write_dir(&path);
    path.push("file.txt");
    utils::write_file(FILE_SIZE, &path);
    c.bench_function("nuke one file", move |b| b.iter(|| nuke::nuke(&path)));
}

criterion_group!(benches, delete_one_file_nuke);
criterion_main!(benches);
