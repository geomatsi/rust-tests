fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if num_rows == 0 {
        return res;
    }

    res.push(vec![1]);
    if num_rows == 1 {
        return res;
    }

    res.push(vec![1, 1]);
    if num_rows == 2 {
        return res;
    }

    for i in 1..(num_rows as usize - 1) {
        let mut v: Vec<i32> = vec![1];

        for j in 0..(res[i].len() - 1) {
            v.push(res[i][j] + res[i][j + 1]);
        }

        v.push(1);
        res.push(v);
    }

    res
}

/* tests */

#[test]
fn pascal_test() {
    assert_eq!(generate(1), vec![vec![1]]);
    assert_eq!(generate(2), vec![vec![1], vec![1, 1]]);
    assert_eq!(generate(3), vec![vec![1], vec![1, 1], vec![1, 2, 1],]);
    assert_eq!(
        generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
    assert_eq!(
        generate(6),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1]
        ]
    );
    assert_eq!(
        generate(7),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1]
        ]
    );
}
