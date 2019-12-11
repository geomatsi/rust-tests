fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 {
        return;
    }

    let n: i32 = nums.len() as i32;
    let k1: i32 = k % n;
    let k2: usize = (n - k1) as usize;

    nums[0..k2].reverse();
    nums[k2..].reverse();
    nums.reverse();
}

/* tests */

#[test]
fn test_n1() {
    let mut nums = vec![];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![]);

    let mut nums = vec![1];
    rotate(&mut nums, 5);
    assert_eq!(nums, vec![1]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 0);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![2, 1]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![1, 2, 3, 4];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![4, 1, 2, 3]);

    let mut nums = vec![1, 2, 3, 4];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 4, 1, 2]);
}
