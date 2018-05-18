//use std::any::Any;

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
