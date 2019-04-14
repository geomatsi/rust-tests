extern crate lazy_static;

fn main() {
    println!("run tests: cargo test --bin test-refs");
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::cell::RefCell;
    use std::ops::DerefMut;
    use std::sync::Mutex;

    lazy_static! {
        static ref GLOBAL: Mutex<Option<u32>> = Mutex::new(None);
        static ref GLOBAL_CELL: Mutex<RefCell<Option<u32>>> = Mutex::new(RefCell::new(None));
    }

    #[test]
    pub fn f_test_global_mutex1() {
        {
            // scoped lock guard: mutex will be unlocked when guard goes out of scope
            let guard = GLOBAL.lock();

            if let Ok(mut n) = guard {
                n.replace(20);
            }

            // first unwrap for guard, second unwrap for Option
            assert_eq!(GLOBAL.lock().unwrap().unwrap(), 20);
        }

        // first unwrap for guard, second unwrap for Option
        assert_eq!(GLOBAL.lock().unwrap().unwrap(), 20);
    }

    #[test]
    pub fn f_test_global_mutex2() {
        {
            // scoped lock guard: mutex will be unlocked when guard goes out of scope
            let guard = GLOBAL_CELL.lock();

            if let Ok(n) = guard {
                // RefCell inner value is replaced
                n.replace(Some(20));
            }

            // unwrap for guard, borrow for RefCell, second unwrap for Option
            assert_eq!(GLOBAL_CELL.lock().unwrap().borrow().unwrap(), 20);
        }

        assert_eq!(GLOBAL_CELL.lock().unwrap().borrow().unwrap(), 20);

        {
            // scoped lock guard: mutex will be unlocked when guard goes out of scope
            let guard = GLOBAL_CELL.lock();

            if let Ok(n) = guard {
                // RefCell inner value dereferenced and updated
                *n.borrow_mut() = Some(30);
            }

            // unwrap for guard, borrow for RefCell, second unwrap for Option
            assert_eq!(GLOBAL_CELL.lock().unwrap().borrow().unwrap(), 30);
        }

        assert_eq!(GLOBAL_CELL.lock().unwrap().borrow().unwrap(), 30);

        {
            // scoped lock guard: mutex will be unlocked when guard goes out of scope
            let guard = GLOBAL_CELL.lock();

            if let Ok(n) = guard {
                // RefCell inner value dereferenced and updated
                n.borrow_mut().deref_mut().replace(30);
            }

            // unwrap for guard, borrow for RefCell, second unwrap for Option
            assert_eq!(GLOBAL_CELL.lock().unwrap().borrow().unwrap(), 30);
        }

        assert_eq!(GLOBAL_CELL.lock().unwrap().borrow().unwrap(), 30);

    }
}
