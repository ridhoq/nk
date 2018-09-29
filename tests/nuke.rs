extern crate nuke;
extern crate rand;

use std::fs;
use std::panic;
use std::path::PathBuf;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn create_dir(path: &PathBuf) {
    fs::create_dir_all(path).expect("couldn't create dir");
}

fn remove_dir(path: &PathBuf) {
    fs::remove_dir_all(path).expect("couldn't remove dir");
}

fn setup() -> PathBuf {
    let root = PathBuf::from(format!("./tmp-{}/", get_rand_string()));
    root
}

fn get_rand_string() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(30).collect()
}

// https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab
fn run_test<T>(test: T) -> ()
where
    T: FnOnce(&PathBuf) -> () + panic::UnwindSafe,
{
    let root = setup();

    let result = panic::catch_unwind(|| test(&root));

    remove_dir(&root);
    assert!(result.is_ok())
}

#[test]
fn it_deletes_an_empty_dir() {
    run_test(|root| {
        let path = root.join("empty-dir");
        create_dir(&path);
        assert!(nuke::nuke(&path).is_ok());
        assert_eq!(path.exists(), false);
    });
}

#[test]
fn it_deletes_a_nested_dir() {
    run_test(|root| {
        let base = root.join("nested-dir");
        let path = base.join("hey").join("wassup").join("hello");
        create_dir(&path);
        assert!(nuke::nuke(&base).is_ok());
        assert_eq!(base.exists(), false);
    });
}

#[test]
fn it_deletes_a_tree_of_nested_dir() {
    run_test(|root| {
        let path = root.join("tree-nested-dir");
        let path1 = path.join("hey").join("wassup").join("hello");
        create_dir(&path1);
        let path2 = path.join("you").join("already").join("know");
        create_dir(&path2);
        let path3 = path.join("pls").join("ok");
        create_dir(&path3);
        let path4 = path
            .join("i")
            .join("had")
            .join("to")
            .join("do")
            .join("it")
            .join("to")
            .join("em");
        create_dir(&path4);
        assert!(nuke::nuke(&path).is_ok());
        assert_eq!(path.exists(), false);
    });
}

#[test]
fn it_does_nothing_if_non_existent_dir() {
    run_test(|root| {
        create_dir(&root);
        let path = root.join("non-existent-dir");
        assert!(nuke::nuke(&path).is_ok());
    })
}
