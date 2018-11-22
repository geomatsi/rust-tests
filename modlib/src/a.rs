use crate::b;

pub fn a_test_func1(v: u32) -> u32 {
    v
}

pub fn a_test_func2(v: u32) -> u32 {
    b::b_test_func1(v)   
}
