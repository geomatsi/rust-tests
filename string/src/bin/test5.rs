fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn reverse_string(s: &mut Vec<char>) {
    // life is short: use std :)
    // s.reverse()

    if s.len() < 2 {
        return;
    }

    let mut n = 0;
    let mut m = s.len() - 1;

    while n < m {
        s.swap(n, m);
        n = n + 1;
        m = m - 1;
    }

    return;
}
/* tests */

#[test]
fn test_n1() {
    let mut s = vec![];
    reverse_string(&mut s);
    assert_eq!(s, vec![]);

    let mut s = vec!['a'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['a']);

    let mut s = vec!['a', 'b'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['b', 'a']);

    let mut s = vec!['a', 'b', 'c'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['c', 'b', 'a']);

    let mut s = vec!['a', 'b', 'c', 'd'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['d', 'c', 'b', 'a']);

    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
}
