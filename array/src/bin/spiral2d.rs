fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    if n == 0 {
        return vec![];
    }

    let m = matrix[0].len();
    if m == 0 {
        return vec![];
    }

    let mut res: Vec<i32> = Vec::new();

    let mut i1 = 0;
    let mut i2 = m - 1;
    let mut j1 = 0;
    let mut j2 = n - 1;

    loop {
        if i1 > i2 {
            break;
        }

        for s in i1..=i2 {
            res.push(matrix[j1][s]);
        }

        if (j1 + 1) > j2 {
            break;
        }

        for s in (j1 + 1)..=j2 {
            res.push(matrix[s][i2]);
        }

        if i1 >= i2 {
            break;
        }

        for s in (i1..i2).rev() {
            res.push(matrix[j2][s]);
        }

        if (j1 + 1) >= j2 {
            break;
        }

        for s in ((j1 + 1)..j2).rev() {
            res.push(matrix[s][i1]);
        }

        i1 += 1;
        i2 = i2.saturating_sub(1);
        j1 += 1;
        j2 = j2.saturating_sub(1);
    }

    res
}

/* tests */

#[test]
fn spiral_test() {
    assert_eq!(spiral_order(vec![vec![]]), vec![]);
    assert_eq!(spiral_order(vec![vec![], vec![]]), vec![]);
    assert_eq!(spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(spiral_order(vec![vec![1, 2]]), vec![1, 2]);
    assert_eq!(spiral_order(vec![vec![1, 2, 3, 4]]), vec![1, 2, 3, 4]);
    assert_eq!(
        spiral_order(vec![vec![1], vec![2], vec![3], vec![4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(spiral_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 4, 3]);
    assert_eq!(
        spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}
