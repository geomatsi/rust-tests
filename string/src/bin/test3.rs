fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn str_str(haystack: String, needle: String) -> i32 {
    let hlen = haystack.len();
    let nlen = needle.len();

    if nlen == 0 {
        return 0;
    }

    if nlen > hlen {
        return -1;
    }

    for i in 0..=(hlen - nlen) {
        let sub = &haystack[i..(nlen + i)];
        if sub == needle {
            return i as i32;
        }
    }

    -1
}

/* tests */

#[test]
fn test_n1() {
    assert_eq!(str_str("hello".to_string(), "".to_string()), 0);
    assert_eq!(str_str("short".to_string(), "tooooolong".to_string()), -1);
    assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
    assert_eq!(str_str("hello".to_string(), "hello".to_string()), 0);
    assert_eq!(str_str("ababab".to_string(), "ab".to_string()), 0);
    assert_eq!(str_str("ababab".to_string(), "ba".to_string()), 1);
}
