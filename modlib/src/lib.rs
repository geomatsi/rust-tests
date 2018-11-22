pub mod a;
pub mod b;

#[test]
fn f_t1() {
    assert_eq!(a::a_test_func1(1), 1);
}

#[test]
fn f_t2() {
    assert_eq!(a::a_test_func2(1), 2);
}

#[test]
fn f_t3() {
    assert_eq!(b::b_test_func1(1), 2);
}

#[test]
fn f_t4() {
    assert_eq!(b::b_test_func2(1), 1);
}
