fn main() {
    println!("run tests: cargo test");
}

/* array partition */

#[allow(dead_code)]

pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut res: Vec<i32> = nums.to_vec();
    let mut sum = 0;
    let mut ind = 0;

    res.sort();

    while ind < res.len() {
        sum += res[ind];
        ind += 2;
    }

    return sum;
}

/* array partition tests */

#[test]
fn ap_test() {
    assert_eq!(array_pair_sum(vec![1, 4, 3, 2]), 4);
    assert_eq!(array_pair_sum(vec![1, 1, 2, 10]), 3);
    assert_eq!(array_pair_sum(vec![1, 10, 90, 100]), 91);
    assert_eq!(array_pair_sum(vec![1, 2, 3, 4, 5, 6]), 9);
}
