fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn find_diag_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    if n == 0 {
        return vec![];
    }

    let m = matrix[0].len();
    if m == 0 {
        return vec![];
    }

    let mut res: Vec<i32> = Vec::with_capacity(n * m);

    for k in 0..(n + m - 1) {
        for i in 0..(k + 1) {
            if k % 2 == 0 {
                if i < m && (k - i) < n {
                    res.push(matrix[k - i][i]);
                }
            } else {
                if i < n && (k - i) < m {
                    res.push(matrix[i][k - i]);
                }
            }
        }
    }

    res
}

/* tests */

#[test]
fn diag_test() {
    assert_eq!(find_diag_order(vec![vec![]]), vec![]);
    assert_eq!(find_diag_order(vec![vec![], vec![]]), vec![]);
    assert_eq!(find_diag_order(vec![vec![1]]), vec![1]);
    assert_eq!(find_diag_order(vec![vec![1, 2]]), vec![1, 2]);
    assert_eq!(find_diag_order(vec![vec![1, 2, 3, 4]]), vec![1, 2, 3, 4]);
    assert_eq!(
        find_diag_order(vec![vec![1], vec![2], vec![3], vec![4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_diag_order(vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_diag_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
}
