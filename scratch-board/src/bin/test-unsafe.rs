fn main() {
    println!("run tests: cargo test --bin test-unsafe");
}

#[cfg(test)]
mod tests {
    use std::slice;

    // unsafe pointer operations

    #[test]
    fn f_test_v1() {
        let mut a = 5;
        let b = &a as *const i32;
        let c = &mut a as *mut i32;

        assert_eq!(a, 5);
        assert_eq!(unsafe { *b }, 5);

        unsafe {
            *c = 6;
        }

        assert_eq!(a, 6);
        assert_eq!(unsafe { *b }, 6);
    }

    // unsafe function

    unsafe fn unsafe_ptr_deref(x: *mut i32) -> i32 {
        *x
    }

    #[test]
    fn f_test_v2() {
        let mut a = 10;
        let b = &mut a as *mut i32;

        let c = unsafe { unsafe_ptr_deref(b) };

        assert_eq!(c, 10);
    }

    // unsafe operations in safe function

    fn test_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
            )
        }
    }

    #[test]
    fn f_test_v3() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v;
        let (a, b) = test_split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // extern functions

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn f_test_v4() {
        let a: i32 = -10;

        assert_eq!(10, unsafe { abs(a) });
    }
}
