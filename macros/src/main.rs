//
//
//

#![allow(unused_macros)]

fn main() {
    println!("run tests: cargo test");
}

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

#[test]
fn f_test_m1() {
    assert_eq!(sum![1], 1);
    assert_eq!(sum![1, 2], 3);
    assert_eq!(sum![1, 2, { 1 + 1 }], 5);
}

macro_rules! create_f_multiplier {
    ($f:ident, $x:expr) => {
        fn $f(v: u32) -> u32 {
            $x * v
        }
    };
}

create_f_multiplier!(foo, 2);
create_f_multiplier!(bar, 4);

#[test]
fn f_test_m2() {
    assert_eq!(foo(1), 2);
    assert_eq!(bar(3), 12);
    assert_eq!(foo({ 2 + 2 }), bar(2));
}

macro_rules! sum2 {
    ($x:expr) => (
        $x
    );
    ($x:expr, $($y:expr), +) => (
        {
            let mut s = $x;
            $ (
                s += $y;
            )+
            s
        }
    )
}

#[test]
fn f_test_m3() {
    assert_eq!(sum2![1], 1);
    assert_eq!(sum2![1u32, 2u32], 3u32);
    assert_eq!(sum2![-1i32, 2i32], 1i32);
    assert_eq!(sum2![1.0], 1.0);
    assert_eq!(sum2![1, 2], 3);
    assert_eq!(sum2![1.0, 2.0, { 1.0 + 1.0 }], 5.0);
}
