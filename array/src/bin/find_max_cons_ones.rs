fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut m: i32 = 0;
    let mut k: i32 = -1;

    for i in 0..nums.len() {
        if nums[i] == 0 {
            k = i as i32;
        }

        let m1: i32 = (i as i32) - k;

        if m1 > m {
            m = m1;
        }
    }

    return m;
}

#[test]
fn tl() {
    assert_eq!(0, find_max_consecutive_ones(vec![]));
    assert_eq!(0, find_max_consecutive_ones(vec![0]));
    assert_eq!(1, find_max_consecutive_ones(vec![1]));
    assert_eq!(1, find_max_consecutive_ones(vec![1, 0]));
    assert_eq!(2, find_max_consecutive_ones(vec![1, 1]));
    assert_eq!(3, find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]));
    assert_eq!(3, find_max_consecutive_ones(vec![1, 1, 1, 0, 1, 1]));
    assert_eq!(5, find_max_consecutive_ones(vec![0, 1, 1, 1, 1, 1, 0]));
}
