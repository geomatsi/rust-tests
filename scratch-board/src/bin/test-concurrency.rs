//
//
//

use std::thread;

fn main() {
    println!("run tests: cargo test --bin test-concurrency");
}

#[test]
fn f_test_thread_closure_v1() {
    let mut a = 5;

    let handle = thread::spawn(move || {
        a = 10;
    });

    handle.join().unwrap();

    assert_eq!(a, 5);
}

//

struct Env {
    key: u32,
}

impl Env {
    fn new(k: u32) -> Env {
        Env { key: k }
    }
}

impl Copy for Env {}

impl Clone for Env {
    fn clone(&self) -> Env {
        Env { key: self.key }
    }
}

#[test]
fn f_test_thread_closure_v2() {
    let mut v = Env::new(10);

    let handle = thread::spawn(move || {
        v.key *= 2;
        assert_eq!(v.key, 20);
    });

    handle.join().unwrap();

    assert_eq!(v.key, 10);
}
