extern crate nuke;

use std::fs;
use std::path::Path;

const ROOT:&str = "./tmp/";

fn join_path(base: &str, join: &str) -> String {
    format!("{b}{j}", b = base.to_string(), j = join.to_string())
}

fn create_dir(path: &str) {
    fs::create_dir_all(path).expect("");
}

#[test]
fn it_deletes_an_empty_dir() {
    let base = "empty-dir";
    let path = join_path(ROOT, base);
    create_dir(&path);
    assert!(nuke::nuke(&path).is_ok());
    assert_eq!(Path::new(&path).exists(), false);
}

#[test]
fn it_deletes_a_nested_dir() {
    let base = "nested-dir/hey/wassup/hello";
    let path = join_path(ROOT, base);
    create_dir(&path);
    assert!(nuke::nuke(&path).is_ok());
    assert_eq!(Path::new(&path).exists(), false);
}

#[test]
fn it_deletes_a_tree_of_nested_dir() {
    let base = "tree-nested-dir/";
    let path = join_path(ROOT, base);
    let path1 = join_path(&path, "hey/wassup/hello");
    create_dir(&path1);
    let path2 = join_path(&path, "you/already/know");
    create_dir(&path2);
    let path3 = join_path(&path, "pls/ok");
    create_dir(&path3);
    let path4 = join_path(&path, "i/had/to/do/it/to/em");
    create_dir(&path4);
    assert!(nuke::nuke(&path).is_ok());
    assert_eq!(Path::new(&path).exists(), false);
}
/*
fn it_deletes_a_tree_of_nested_dir() {
    let base = "./tree-nested-dir/";
    let path1 = format!("{}{}", base, "hey/wassup/hello");
    fs::create_dir_all(path1);
    let path2 = format!("{}{}", base, "you/already/know");
    fs::create_dir_all(path2);
    let path3 = format!("{}{}", base, "pls/ok");
    fs::create_dir_all(path3);
    let path4 = format!("{}{}", base, "i/had/to/do/it/to/em");
    fs::create_dir_all(path4);
    assert!(nuke::nuke(base).is_ok());
    assert_eq!(Path::new(base).exists(), false);
}*/