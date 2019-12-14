fn main() {
    // concat options
    let s1 = String::from("one");
    let s2 = String::from("two");
    let s3 = String::from("three");

    // can't use since s1 will be 'moved':
    //let s = s1 + &s2 + &s3;

    // format doesn't take ownership of its params:
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{} {} {} {}", s, s1, s2, s3);

    // UTF-8 !!!
    let se = String::from("hello");
    let sr = String::from("Салют");

    // crash! not at UTF-8 symbol boundary:
    //let ss = &sr[0..1];

    println!("{} : len {}", se, se.len());
    println!("{} : len {}", sr, sr.len());

    f_dump_as_chars(&sr);
    f_dump_as_bytes(&sr);

    println!("for more examples run tests: cargo test");
}

// dump string chars
fn f_dump_as_chars(s: &String) {
    let mut t = String::new();

    for c in s.chars() {
        if t.len() == 0 {
            t = format!("{}", c);
        } else {
            t = format!("{}-{}", t, c);
        }
    }

    println!("{}", t);
}

// dump string bytes
fn f_dump_as_bytes(s: &String) {
    let mut t = String::new();

    for c in s.bytes() {
        if t.len() == 0 {
            t = format!("{}", c);
        } else {
            t = format!("{}-{}", t, c);
        }
    }

    println!("{}", t);
}

// example: Hamming distance
// len(s1) == len(s2) => hamming_distance(s1, s2)
// len(s1) != len(s2) => -1
#[allow(dead_code)]
fn hamming_dist(s1: &str, s2: &str) -> i32 {
    if s1.len() != s2.len() {
        return -1;
    }

    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let mut dist = 0;

    for i in 0..s1.len() {
        if b1[i] != b2[i] {
            dist += 1;
        }
    }

    dist
}

#[test]
fn test_hamming_dist() {
    assert_eq!(hamming_dist("a", "ab"), -1);
    assert_eq!(hamming_dist("a", "a"), 0);
    assert_eq!(hamming_dist("abc", "cba"), 2);
    assert_eq!(hamming_dist("abba", "baab"), 4);
}

// example: count words
#[allow(dead_code)]
fn count_words(ss: &str) -> u32 {
    enum WState {
        START,
        WORD,
    }

    let b = ss.as_bytes();
    let mut w: u32 = 0;
    let mut state: WState = WState::START;

    for i in 0..ss.len() {
        match state {
            WState::START => {
                if b[i] != b' ' {
                    state = WState::WORD;
                    w += 1;
                }
            }
            WState::WORD => {
                if b[i] == b' ' {
                    state = WState::START;
                }
            }
        }
    }

    return w;
}

#[test]
fn test_count_words() {
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words(" "), 0);
    assert_eq!(count_words("a"), 1);
    assert_eq!(count_words("aaa"), 1);
    assert_eq!(count_words(" aaa   "), 1);
    assert_eq!(count_words(" aa   bbb"), 2);
    assert_eq!(count_words(" aa   bbb c  "), 3);
    assert_eq!(count_words("a a   bbb c ccc"), 5);
}

// example: replace char
#[allow(dead_code)]
fn replace_char(s: &str, co: char, cn: char) -> String {
    let mut res = String::from("");
    let chars = s.chars();

    for c in chars {
        if c == co {
            res.push(cn);
        } else {
            res.push(c);
        }
    }

    return res;
}

#[test]
fn test_replace_char() {
    assert_eq!(replace_char("a", 'x', 'y'), "a");
    assert_eq!(replace_char("a", 'a', 'b'), "b");
    assert_eq!(replace_char("a b c", ' ', '_'), "a_b_c");
    assert_eq!(replace_char("abab", 'a', '\u{40}'), "@b@b");
}

// example: add two binary strings
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

#[test]
fn test_add_binary() {
    assert_eq!(add_binary("0".to_string(), "0".to_string()), "0");
    assert_eq!(add_binary("1".to_string(), "0".to_string()), "1");
    assert_eq!(add_binary("1".to_string(), "1".to_string()), "10");
    assert_eq!(add_binary("11".to_string(), "11".to_string()), "110");
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
    assert_eq!(add_binary("111".to_string(), "1111".to_string()), "10110");
}

// example: find substring
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

#[test]
fn test_str_str() {
    assert_eq!(str_str("hello".to_string(), "".to_string()), 0);
    assert_eq!(str_str("short".to_string(), "tooooolong".to_string()), -1);
    assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
    assert_eq!(str_str("hello".to_string(), "hello".to_string()), 0);
    assert_eq!(str_str("ababab".to_string(), "ab".to_string()), 0);
    assert_eq!(str_str("ababab".to_string(), "ba".to_string()), 1);
}

// example: find the longest common prefix
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

#[test]
fn test_lcp() {
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

// example: reverse string
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

#[test]
fn test_reverse_string() {
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

// example: reverse words order in strings trimming all extra spaces
#[allow(dead_code)]
fn reverse_words(s: String) -> String {
    if s.len() == 0 {
        return s;
    }

    let c = s.chars().collect::<Vec<char>>();
    let mut v: Vec<char> = Vec::new();

    let mut i = c.len() - 1;
    let mut j;

    loop {
        loop {
            if i == 0 || !c[i].is_whitespace() {
                break;
            }

            i -= 1;
        }

        if i == 0 && c[i].is_whitespace() {
            break;
        }

        if !v.is_empty() {
            v.push(' ');
        }

        j = i;

        loop {
            if j == 0 || c[j].is_whitespace() {
                break;
            }

            j -= 1;
        }

        if j == 0 && !c[j].is_whitespace() {
            for k in j..i + 1 {
                v.push(c[k]);
            }

            break;
        }

        for k in j + 1..i + 1 {
            v.push(c[k]);
        }

        i = j;
    }

    v.iter().collect::<String>()
}

#[test]
fn test_reverse_words() {
    assert_eq!("".to_string(), reverse_words("".to_string()));
    assert_eq!("".to_string(), reverse_words(" ".to_string()));
    assert_eq!("".to_string(), reverse_words("    ".to_string()));
    assert_eq!("c b a".to_string(), reverse_words("a b c".to_string()));
    assert_eq!("c b a".to_string(), reverse_words(" a b  c  ".to_string()));
    assert_eq!("blue".to_string(), reverse_words("blue".to_string()));
    assert_eq!("слово".to_string(), reverse_words("слово".to_string()));
    assert_eq!(
        "дело и слово".to_string(),
        reverse_words("слово и дело".to_string())
    );
    assert_eq!(
        "дело и слово".to_string(),
        reverse_words("  слово  и дело   ".to_string())
    );
    assert_eq!(
        "blue is sky the".to_string(),
        reverse_words("the sky is blue".to_string())
    );
    assert_eq!(
        "example good a".to_string(),
        reverse_words("a good   example".to_string())
    );
    assert_eq!(
        "world! hello".to_string(),
        reverse_words("  hello world!  ".to_string())
    );
}

// example: reverse each word in string
#[allow(dead_code)]
fn reverse_each_word(s: String) -> String {
    if s.len() == 0 {
        return s;
    }

    let c = s.chars().collect::<Vec<char>>();
    let mut v: Vec<char> = Vec::new();

    let mut i = 0;
    let mut j;

    loop {
        loop {
            if i == c.len() - 1 || !c[i].is_whitespace() {
                break;
            }

            i += 1;
        }

        if i == c.len() - 1 && c[i].is_whitespace() {
            break;
        }

        if !v.is_empty() {
            v.push(' ');
        }

        j = i;

        loop {
            if j == c.len() - 1 || c[j].is_whitespace() {
                break;
            }

            j += 1;
        }

        if j == c.len() - 1 && !c[j].is_whitespace() {
            for k in (i..j + 1).rev() {
                v.push(c[k]);
            }

            break;
        }

        for k in (i..j).rev() {
            v.push(c[k]);
        }

        i = j;
    }

    v.iter().collect::<String>()
}

#[test]
fn test_reverse_each_word() {
    assert_eq!("".to_string(), reverse_each_word("".to_string()));
    assert_eq!("".to_string(), reverse_each_word(" ".to_string()));
    assert_eq!("".to_string(), reverse_each_word("    ".to_string()));
    assert_eq!("a b c".to_string(), reverse_each_word("a b c".to_string()));
    assert_eq!(
        "a b c".to_string(),
        reverse_each_word(" a b  c  ".to_string())
    );
    assert_eq!("оволс".to_string(), reverse_each_word("слово".to_string()));
    assert_eq!(
        "оволс и олед".to_string(),
        reverse_each_word("слово и дело".to_string())
    );
    assert_eq!(
        "оволс и олед".to_string(),
        reverse_each_word("   слово  и дело ".to_string())
    );
    assert_eq!("eulb".to_string(), reverse_each_word("blue".to_string()));
    assert_eq!(
        "eht yks si eulb".to_string(),
        reverse_each_word("the sky is blue".to_string())
    );
    assert_eq!(
        "a doog elpmaxe".to_string(),
        reverse_each_word("a good   example".to_string())
    );
    assert_eq!(
        "olleh !dlrow".to_string(),
        reverse_each_word("  hello world!  ".to_string())
    );
}
