fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut p: Vec<i32> = digits.to_vec();
    let r = p.len();
    let mut z = 0;

    p[r - 1] += 1;

    for idx in (0..r).rev() {
        p[idx] += z;
        if p[idx] >= 10 {
            p[idx] -= 10;
            z = 1;
        } else {
            z = 0;
            break;
        }
    }

    if z == 1 {
        p.insert(0, 1);
    }

    return p;
}

/* tests */

#[test]
fn po_test() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![0]), vec![1]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
    assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    assert_eq!(plus_one(vec![4, 3, 3, 1]), vec![4, 3, 3, 2]);
}
