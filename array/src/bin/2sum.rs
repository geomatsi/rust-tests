fn main() {
    println!("run tests: cargo test");
}

/* array partition */

#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut j = (numbers.len() - 1) as usize;
    let mut i = 0 as usize;

    while i < j {
        let sum = numbers[i] + numbers[j];

        if sum < target {
            i += 1;
        } else if sum > target {
            j -= 1;
        } else {
            break;
        }
    }

    return vec![(i + 1) as i32, (j + 1) as i32];
}

/* array partition tests */

#[test]
fn ap_test() {
    assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![2, 4], two_sum(vec![1, 2, 3, 4, 6], 6));
    assert_eq!(vec![2, 3], two_sum(vec![1, 10, 21, 22, 31, 40], 31));
}
