fn main() {
    println!("run tests: cargo test --bin test-concurrency");
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    // thread closure

    #[test]
    fn f_test_thread_closure_v1() {
        let mut a = 5;

        let handle = thread::spawn(move || {
            a = 10;
        });

        handle.join().unwrap();

        assert_eq!(a, 5);
    }

    // thread closure

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

    // messages

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

    // mutexes

    #[test]
    fn f_test_mutexes_v1() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10u32);
    }
}
