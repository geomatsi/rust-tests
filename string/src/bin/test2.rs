fn main() {
    println!("run tests: cargo test");
}

#[allow(dead_code)]
fn add_binary(a: String, b: String) -> String {
    let mut av = a;
    let mut bv = b;
    let mut cv = Vec::new();

    let mut z = 0;

    let n = if av.len() > bv.len() {
        av.len()
    } else {
        bv.len()
    };

    for i in 0..=n {
        let s1 = match av.pop() {
            Some('1') => 1,
            _ => 0,
        };
        let s2 = match bv.pop() {
            Some('1') => 1,
            _ => 0,
        };

        let s3 = s1 + s2 + z;

        match s3 {
            0 => {
                // drop leading zero
                if i < n {
                    cv.insert(0, '0');
                    z = 0;
                }
            }
            1 => {
                cv.insert(0, '1');
                z = 0;
            }
            2 => {
                cv.insert(0, '0');
                z = 1;
            }
            3 => {
                cv.insert(0, '1');
                z = 1;
            }
            _ => unreachable!("should not be here"),
        }
    }

    cv.iter().collect::<String>()
}

/* tests */

#[test]
fn test_n1() {
    assert_eq!(add_binary("0".to_string(), "0".to_string()), "0");
    assert_eq!(add_binary("1".to_string(), "0".to_string()), "1");
    assert_eq!(add_binary("1".to_string(), "1".to_string()), "10");
    assert_eq!(add_binary("11".to_string(), "11".to_string()), "110");
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
    assert_eq!(add_binary("111".to_string(), "1111".to_string()), "10110");
}
