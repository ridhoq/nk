extern crate nuke;

use std::fs;
use std::panic;
use std::path::Path;

const ROOT: &str = "./tmp/";

fn join_path(base: &str, join: &str) -> String {
    format!("{b}{j}", b = base.to_string(), j = join.to_string())
}

fn create_dir(path: &str) {
    fs::create_dir_all(path).expect("");
}

fn setup() {
    fs::remove_dir_all(ROOT);
}

// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + panic::UnwindSafe,
{
    setup();

    let result = panic::catch_unwind(|| test());

    assert!(result.is_ok())
}

#[test]
fn it_deletes_an_empty_dir() {
    run_test(move || {
        let base = "empty-dir";
        let path = join_path(ROOT, base);
        create_dir(&path);
        assert!(nuke::nuke(&path).is_ok());
        assert_eq!(Path::new(&path).exists(), false);
    });
}

#[test]
fn it_deletes_a_nested_dir() {
    run_test(move || {
        let base = join_path(ROOT, "nested-dir/");
        let path = join_path(&base, "hey/wassup/hello");
        create_dir(&path);
        assert!(nuke::nuke(&base).is_ok());
        assert_eq!(Path::new(&base).exists(), false);
    });
}

#[test]
fn it_deletes_a_tree_of_nested_dir() {
    run_test(move || {
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
    });
}
