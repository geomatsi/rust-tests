use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

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

// lists using Rc

enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

use ListRc::{ConsRc, NilRc};

impl ListRc {
    fn cons_head(&self) -> Option<i32> {
        match self {
            ConsRc(head, _) => Some(*head),
            NilRc => None,
        }
    }

    fn cons_tail(&self) -> Option<Rc<ListRc>> {
        match self {
            ConsRc(_, tail) => Some(Rc::clone(tail)),
            NilRc => None,
        }
    }
}

#[test]
fn f_test_rc_list() {
    let a = ConsRc(2, Rc::new(ConsRc(1, Rc::new(NilRc))));
    let ra = Rc::new(a);
    let wa = Rc::downgrade(&ra);

    let b = ConsRc(10, Rc::clone(&ra));
    let rb = Rc::new(b);

    let c = ConsRc(100, Rc::clone(&rb));
    let rc = Rc::new(c);

    let d = ConsRc(1000, Rc::clone(&ra));
    let rd = Rc::new(d);

    assert_eq!(ra.cons_head().unwrap(), 2);

    assert_eq!(rb.cons_head().unwrap(), 10);
    assert_eq!(rb.cons_tail().unwrap().cons_head().unwrap(), 2);

    assert_eq!(rc.cons_head().unwrap(), 100);
    assert_eq!(rc.cons_tail().unwrap().cons_head().unwrap(), 10);
    assert_eq!(
        rc.cons_tail().unwrap().cons_tail().unwrap().cons_head(),
        Some(2)
    );

    assert_eq!(rd.cons_head().unwrap(), 1000);
    assert_eq!(rd.cons_tail().unwrap().cons_head().unwrap(), 2);

    assert!(wa.upgrade().is_some());

    drop(ra);

    assert_eq!(rb.cons_head().unwrap(), 10);
    assert_eq!(rb.cons_tail().unwrap().cons_head().unwrap(), 2);

    assert_eq!(rc.cons_head().unwrap(), 100);
    assert_eq!(rc.cons_tail().unwrap().cons_head().unwrap(), 10);
    assert_eq!(
        rc.cons_tail().unwrap().cons_tail().unwrap().cons_head(),
        Some(2)
    );

    assert_eq!(rd.cons_head().unwrap(), 1000);
    assert_eq!(rd.cons_tail().unwrap().cons_head().unwrap(), 2);

    assert!(wa.upgrade().is_some());

    drop(rb);

    assert_eq!(rc.cons_head().unwrap(), 100);
    assert_eq!(rc.cons_tail().unwrap().cons_head().unwrap(), 10);
    assert_eq!(
        rc.cons_tail().unwrap().cons_tail().unwrap().cons_head(),
        Some(2)
    );

    assert!(wa.upgrade().is_some());

    drop(rc);

    assert_eq!(rd.cons_head().unwrap(), 1000);
    assert_eq!(rd.cons_tail().unwrap().cons_head().unwrap(), 2);

    assert!(wa.upgrade().is_some());

    drop(rd);

    assert!(wa.upgrade().is_none());
}

// RefCell runtime checks

#[test]
fn f_test_refcell_borrow() {
    let a = RefCell::new(String::from("a"));

    {
        let b = a.borrow();
        let c = a.borrow();

        assert_eq!(a.borrow().as_str(), "a");
        assert_eq!(b.as_str(), "a");
        assert_eq!(c.as_str(), "a");
    }

    a.borrow_mut().push('a');

    let b = a.borrow();
    let c = a.borrow();

    assert_eq!(a.borrow().as_str(), "aa");
    assert_eq!(b.as_str(), "aa");
    assert_eq!(c.as_str(), "aa");
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn f_test_refcell_borrow_unmut_mut() {
    let a = RefCell::new(String::from("a"));
    let b = a.borrow();

    a.borrow_mut().push('a');

    assert_eq!(b.as_str(), "a");
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn f_test_refcell_borrow_mut_mut() {
    let a = RefCell::new(String::from("a"));
    let b = a.borrow_mut();

    a.borrow_mut().push('a');

    assert_eq!(b.as_str(), "a");
}

// interior mutability using RefCell

pub trait MQTTpub {
    fn publish(&self, topic: &str, msg: &str);
}

struct MockMQTTpublisher {
    posts: RefCell<HashMap<String, String>>,
    count: RefCell<u32>,
}

impl MockMQTTpublisher {
    fn new() -> MockMQTTpublisher {
        MockMQTTpublisher {
            posts: RefCell::new(HashMap::new()),
            count: RefCell::new(0u32),
        }
    }
}

impl MQTTpub for MockMQTTpublisher {
    // note that self is not mutable in publish, but borrow_mut is used to implement mock tracking
    fn publish(&self, topic: &str, msg: &str) {
        self.posts
            .borrow_mut()
            .insert(String::from(topic), String::from(msg));
        *self.count.borrow_mut() += 1;
    }
}

#[test]
fn f_test_refcell_interior_mutability() {
    let mock_publisher = MockMQTTpublisher::new();

    mock_publisher.publish("topic1", "hello");
    mock_publisher.publish("topic2", "world");

    assert_eq!(*mock_publisher.count.borrow(), 2);
}
