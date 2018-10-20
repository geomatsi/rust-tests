//
//
//

#![allow(unused_macros)]

macro_rules! sum {
    ( $($x:expr), *) => {
        {
        let mut s = 0;
        $ (
            s += $x;
        )*
        s
        }
    };
}

fn main() {
    println!("run tests: cargo test");
}

#[test]
fn f_test_m1() {
    assert_eq!(sum![1], 1);
    assert_eq!(sum![1, 2], 3);
    assert_eq!(sum![1, 2, { 1 + 1 }], 5);
}
