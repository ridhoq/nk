extern crate nuke;
extern crate rand;

use std::fs;
use std::panic;
use std::path::Path;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn join_path(base: &str, join: &str) -> String {
    format!("{b}{j}", b = base.to_string(), j = join.to_string())
}

fn create_dir(path: &str) {
    fs::create_dir_all(path).expect(&format!("couldn't create dir: {}", &path));
}

fn remove_dir(path: &str) {
    fs::remove_dir_all(path).expect(&format!("couldn't remove dir: {}", &path));
}

fn setup() -> String {
    let root = format!("./tmp-{}/", get_rand_string());
    root
}

fn get_rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect()
}

// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
fn run_test<T>(test: T) -> ()
where
    T: FnOnce(&str) -> () + panic::UnwindSafe,
{
    let root = setup();

    let result = panic::catch_unwind(|| test(&root));

    remove_dir(&root);
    assert!(result.is_ok())
}

#[test]
fn it_deletes_an_empty_dir() {
    run_test(|root| {
        let base = "empty-dir";
        let path = join_path(&root, base);
        create_dir(&path);
        assert!(nuke::nuke(&path).is_ok());
        assert_eq!(Path::new(&path).exists(), false);
    });
}

#[test]
fn it_deletes_a_nested_dir() {
    run_test(|root| {
        let base = join_path(&root, "nested-dir/");
        let path = join_path(&base, "hey/wassup/hello");
        create_dir(&path);
        assert!(nuke::nuke(&base).is_ok());
        assert_eq!(Path::new(&base).exists(), false);
    });
}

#[test]
fn it_deletes_a_tree_of_nested_dir() {
    run_test(|root| {
        let base = "tree-nested-dir/";
        let path = join_path(&root, base);
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

#[test]
fn it_does_nothing_if_non_existent_dir() {
    run_test(|root| {
        create_dir(&root);
        let base = "non-existent-dir/";
        let path = join_path(&root, base);
        assert!(nuke::nuke(&path).is_ok());
    })
}
