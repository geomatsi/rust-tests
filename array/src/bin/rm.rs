fn main() {
    println!("run tests: cargo test");
}

/* array partition */

#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    return k;
}

/* array partition tests */

#[test]
fn ap_test() {
    let mut num = vec![3, 2, 2, 3];
    let len = remove_element(&mut num, 3);
    num.resize(len, 0);
    assert_eq!(len, 2);
    assert_eq!(vec![2, 2], num);

    let mut num = vec![2, 3, 2, 3];
    let len = remove_element(&mut num, 3);
    num.resize(len, 0);
    assert_eq!(len, 2);
    assert_eq!(vec![2, 2], num);

    let mut num = vec![2, 2, 3, 3];
    let len = remove_element(&mut num, 3);
    num.resize(len, 0);
    assert_eq!(len, 2);
    assert_eq!(vec![2, 2], num);

    let mut num = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let len = remove_element(&mut num, 2);
    num.resize(len, 0);
    assert_eq!(len, 5);
    assert_eq!(vec![0, 1, 3, 0, 4], num);
}
