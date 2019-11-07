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

    let n = s.len();
    let m = s.len() / 2;

    for i in 0..m {
        s.swap(i, n - 1 - i);
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
