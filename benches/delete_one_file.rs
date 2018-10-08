#[macro_use]
extern crate criterion;
extern crate nuke;

use criterion::Criterion;
use criterion::Fun;
use std::fs;
use std::path::PathBuf;
mod utils;

const FILE_SIZE: usize = 500000;

fn delete_one_file(c: &mut Criterion) {
    let path = write_one_file("../delete_one_file");
    let nuke_fun = Fun::new("nuke", move |b, p| b.iter(|| nuke::nuke(&p)));
    let std_fun = Fun::new("std", move |b, p| b.iter(|| fs::remove_dir_all(&p)));
    let functions = vec!(nuke_fun, std_fun);
    c.bench_functions("delete_one_file", functions, path);
}

fn write_one_file(base_path: &str) -> PathBuf {
    let mut path = PathBuf::from(base_path);
    utils::write_dir(&path);
    path.push("file.txt");
    utils::write_file(FILE_SIZE, &path);
    path
}

fn delete_one_hundred_files(c: &mut Criterion) {
    let path = write_one_hundred_files("../delete_many_files");
    let nuke_fun = Fun::new("nuke", move |b, p| b.iter(|| nuke::nuke(&p)));
    let std_fun = Fun::new("std", move |b, p| b.iter(|| fs::remove_dir_all(&p)));
    let functions = vec!(nuke_fun, std_fun);
    c.bench_functions("delete_one_hundred_files", functions, path);
}

fn write_one_hundred_files(base_path: &str) -> PathBuf {
    let path = PathBuf::from(base_path);
    utils::write_dir(&path);
    for i in 0..100 {
        let mut file_path = PathBuf::from(&path);
        file_path.push(format!("{}.txt", i));
        utils::write_file(FILE_SIZE, &file_path);
    }
    path
}

criterion_group!(benches, delete_one_file, delete_one_hundred_files);
criterion_main!(benches);
