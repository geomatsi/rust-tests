fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = "";

    if strs.is_empty() {
        return String::from(res);
    }

    let xs = &strs[0];
    for n in 1..=xs.len() {
        let prefix = &xs[0..n];
        for s in strs.iter() {
            if s.len() < n || xs[n - 1..n] != s[n - 1..n] {
                return String::from(res);
            }
        }

        res = prefix;
    }

    String::from(res)
}

/* tests */

#[test]
fn test_n1() {
    assert_eq!(longest_common_prefix(vec![]), "".to_string());
    assert_eq!(
        longest_common_prefix(vec!["aaa".to_string(), "aa".to_string()]),
        "aa".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "aaaaa".to_string(),
            "bbbbbbb".to_string(),
            "ccccccccccc".to_string()
        ]),
        "".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "abcd".to_string(),
            "abcdabcd".to_string(),
            "abcdabcdabcd".to_string()
        ]),
        "abcd".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec![
            "axxxxxx".to_string(),
            "ayyyyyyy".to_string(),
            "abcd".to_string()
        ]),
        "a".to_string()
    );
}
