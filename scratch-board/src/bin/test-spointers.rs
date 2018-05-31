use std::cell::RefCell;
use std::ops::Deref;

//

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);

    //

    println!("run tests: cargo test --bin test-iterators");
}

//

#[test]
fn f_test_box() {
    let v1 = Box::new(5u32);
    let v2 = Box::new("hello");

    unsafe {
        assert_eq!(*Box::into_raw(v1), 5u32);
        assert_eq!(*Box::into_raw(v2), "hello");
    }
}

//

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn cons_head(v: &List) -> Option<i32> {
    match *v {
        Cons(head, _) => Some(head),
        Nil => None,
    }
}

fn cons_tail(v: List) -> Option<List> {
    match v {
        Cons(_, tail) => Some(*tail),
        Nil => None,
    }
}

#[test]
fn f_test_head() {
    let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list2 = Nil;

    assert_eq!(cons_head(&list1), Some(1));
    assert_eq!(cons_head(&list1), Some(1));
    assert_eq!(cons_head(&list2), None);
    assert_eq!(cons_head(&list2).is_none(), true);
}

#[test]
fn f_test_tail() {
    let list1 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list2 = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let list3 = Nil;

    assert_eq!(
        cons_head(&cons_tail(cons_tail(list1).unwrap()).unwrap()),
        Some(3)
    );
    assert_eq!(cons_head(&cons_tail(list2).unwrap()), Some(2));
    assert_eq!(cons_tail(list3).is_none(), true);
}

#[test]
fn f_test_ref_deref() {
    let mut x = 5;

    {
        let y = &x;

        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }

    x += 1;

    {
        let y = &x;

        assert_eq!(x, 6);
        assert_eq!(*y, 6);
    }

    let mut x = 5;
    let z = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*z, 5);

    x += 1;

    assert_eq!(x, 6);
    assert_eq!(*z, 5);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: &T) -> MyBox<T>
    where
        T: std::clone::Clone,
    {
        MyBox(x.clone())
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn f_test_custom_ref_deref1() {
    let mut x = 5;
    let y = MyBox::new(&x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    x += 1;

    assert_eq!(6, x);
    assert_eq!(5, *y);
}

#[test]
fn f_test_custom_ref_deref2() {
    let mut s = String::from("hello");
    let r = MyBox::new(&s);

    assert_eq!("hello", s);
    assert_eq!("hello", *r);

    s.push('w');

    assert_eq!("hellow", s);
    assert_eq!("hello", *r);
}

// verify Drop using counter based on RefCell

struct TestDrop<'a> {
    s: String,
    c: &'a RefCell<u32>,
}

impl<'a> TestDrop<'a> {
    fn new(s: &str, c: &'a RefCell<u32>) -> TestDrop<'a> {
        *c.borrow_mut() += 1;

        TestDrop {
            s: String::from(s),
            c: c,
        }
    }

    fn get_data(&self) -> &String {
        &self.s
    }
}

impl<'a> Drop for TestDrop<'a> {
    fn drop(&mut self) {
        *self.c.borrow_mut() -= 1;
    }
}

#[test]
fn f_test_drop() {
    let c = RefCell::new(0u32);
    assert_eq!(*c.borrow(), 0);

    let v1 = TestDrop::new("v1", &c);

    assert_eq!(v1.get_data().as_str(), "v1");
    assert_eq!(*c.borrow(), 1);

    {
        let v2 = TestDrop::new("v2", &c);

        assert_eq!(v2.get_data().as_str(), "v2");
        assert_eq!(*c.borrow(), 2);
    }

    assert_eq!(v1.get_data().as_str(), "v1");
    assert_eq!(*c.borrow(), 1);

    drop(v1);

    assert_eq!(*c.borrow(), 0);
}
