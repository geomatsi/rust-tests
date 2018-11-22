extern crate modlib as lib;

fn main() {
    assert_eq!(lib::a::a_test_func1(1), 1);
    assert_eq!(lib::a::a_test_func2(1), 2);
    assert_eq!(lib::b::b_test_func1(1), 2);
    assert_eq!(lib::b::b_test_func2(1), 1);

    println!("OK!");
}
