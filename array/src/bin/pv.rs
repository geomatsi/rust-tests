fn main() {
    println!("run tests: cargo test");
}

/* pivot index */

#[allow(dead_code)]
fn pv_idx(num: Vec<i32>) -> i32 {
    if num.len() < 3 {
        return -1;
    }

    let mut s1: i32 = 0;
    let mut s2: i32 = num.iter().sum();

    for (idx, val) in num.iter().enumerate() {
        if s1 == s2 - val {
            return idx as i32;
        }

        s1 += val;
        s2 -= val;
    }

    return -1i32;
}

/* pivot tests */

#[test]
fn pv_t1() {
    assert_eq!(pv_idx(vec![]), -1);
    assert_eq!(pv_idx(vec![1]), -1);
    assert_eq!(pv_idx(vec![1, 2]), -1);
}

#[test]
fn pv_t2() {
    assert_eq!(pv_idx(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(pv_idx(vec![1, 1, 1, 1, 1, 1, 1]), 3);
    assert_eq!(pv_idx(vec![-7, 1, 5, 2, -4, 3, 0]), 3);
    assert_eq!(pv_idx(vec![-1, -1, -1, 0, 1, 1]), 0);
    assert_eq!(pv_idx(vec![-1, -1, 0, 1, 1, 0]), 5);
}

#[test]
fn pv_t3() {
    assert_eq!(pv_idx(vec![1, 2, 3]), -1);
    assert_eq!(pv_idx(vec![1, 2, 3, 4]), -1);
}
