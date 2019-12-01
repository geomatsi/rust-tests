fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let mut xn: usize = nums.len() + 1;
    let mut xs: i32 = 0;
    let mut k: usize = 0;
    let mut m: usize = 0;

    loop {
        if xs < s {
            if m >= nums.len() {
                break;
            }
            xs += nums[m];
            m += 1;
        }

        if xs >= s {
            if (m - k) < xn {
                xn = m - k;
            }

            xs -= nums[k];
            k += 1;
        }
    }

    if xn > nums.len() {
        return 0;
    }

    return xn as i32;
}

#[test]
fn t1() {
    assert_eq!(0, min_sub_array_len(1, vec![]));
    assert_eq!(0, min_sub_array_len(1, vec![0]));
    assert_eq!(1, min_sub_array_len(1, vec![1]));
    assert_eq!(1, min_sub_array_len(1, vec![1, 0, 1, 0]));
    assert_eq!(3, min_sub_array_len(2, vec![1, 0, 1, 0]));
    assert_eq!(4, min_sub_array_len(4, vec![1, 1, 1, 1]));
    assert_eq!(3, min_sub_array_len(4, vec![2, 1, 1, 0, 0, 1]));
    assert_eq!(2, min_sub_array_len(4, vec![2, 1, 1, 0, 0, 1, 3]));
    assert_eq!(2, min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
}
