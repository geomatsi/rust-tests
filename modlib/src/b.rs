use crate::a;

pub fn b_test_func1(v: u32) -> u32 {
    v*2
}

pub fn b_test_func2(v: u32) -> u32 {
    a::a_test_func1(v)
}
