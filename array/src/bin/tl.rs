fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn tl_idx(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return -1;
    }

    if nums.len() == 1 {
        return 0;
    }

    let mut p1: usize = 0;
    let mut v2: i32 = 0;

    for (idx, val) in nums[1..].iter().enumerate() {
        if *val > nums[p1] {
            if nums[p1] > v2 {
                v2 = nums[p1]
            }

            p1 = idx + 1;
        } else {
            if *val > v2 {
                v2 = *val;
            }
        }
    }

    if 2 * v2 <= nums[p1] {
        return p1 as i32;
    }

    return -1i32;
}

/* tests */

#[test]
fn tl_t1() {
    assert_eq!(tl_idx(vec![]), -1);
}

#[test]
fn tl_t2() {
    assert_eq!(tl_idx(vec![2, 3]), -1);
    assert_eq!(tl_idx(vec![3, 2, 1, 0]), -1);
    assert_eq!(tl_idx(vec![3, 3, 1, 5]), -1);
}

#[test]
fn tl_t3() {
    assert_eq!(tl_idx(vec![1]), 0);
    assert_eq!(tl_idx(vec![1, 2]), 1);
    assert_eq!(tl_idx(vec![3, 6, 1, 0]), 1);
    assert_eq!(tl_idx(vec![6, 3, 1, 0]), 0);
    assert_eq!(tl_idx(vec![1, 3, 1, 7]), 3);
}
