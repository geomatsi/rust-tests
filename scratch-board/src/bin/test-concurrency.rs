//
//
//

use std::sync::mpsc;
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

//

#[test]
fn f_test_msg_v1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let m = String::from("one");
        tx.send(m).unwrap();
    });

    assert_eq!(rx.recv().unwrap(), "one");
}

#[test]
fn f_test_msg_v2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let ms = vec![1, 2, 3];

        for m in ms {
            tx.send(m).unwrap();
        }
    });

    // NB: extra element here is ok, since
    //     iterator will end when the channel is closed
    let mut ms = vec![4, 3, 2, 1];

    for m in rx {
        assert_eq!(m, ms.pop().unwrap());
    }
}
