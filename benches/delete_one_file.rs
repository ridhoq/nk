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
    c.bench_function("delete_one_file_nuke", move |b| b.iter(|| nuke::nuke(&path)));
}

fn delete_one_hundred_files_nuke(c: &mut Criterion) {
    let path = PathBuf::from("../delete_many_files/nuke");
    utils::write_dir(&path);
    for i in 0..100 {
        let mut file_path = PathBuf::from(&path);
        file_path.push(format!("{}.txt", i));
        utils::write_file(FILE_SIZE, &file_path);
    }

    c.bench_function("delete_one_hundred_files_nuke", move |b| b.iter(|| nuke::nuke(&path)));
}

criterion_group!(benches, delete_one_file_nuke, delete_one_hundred_files_nuke);
criterion_main!(benches);
