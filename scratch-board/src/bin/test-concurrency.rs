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
