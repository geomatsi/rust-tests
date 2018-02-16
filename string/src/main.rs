fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("wewwo");

    println!("{} {} -> {}", s1, s2, f_hdist(&s1, &s2));

    let s = String::from("This is a short sentence");

    println!("{} -> {} words", s, f_words(&s));

    let s1 = String::from("need to replace spaces");
    let s2 = f_replace(&s, ' ', '_');
    println!("{} -> {}", s1, s2);
}

// f_hdist(s1, s2)
// len(s1) == len(s2) => hamming_distance(s1, s2)
// len(s1) != len(s2) => -1
fn f_hdist(s1: &str, s2: &str) -> i32 {
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
fn f_hdist_test() {
    assert_eq!(f_hdist("a", "ab"), -1);
    assert_eq!(f_hdist("a", "a"), 0);
    assert_eq!(f_hdist("abc", "cba"), 2);
    assert_eq!(f_hdist("abba", "baab"), 4);
}

// words
fn f_words(ss: &str) -> u32 {
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
fn f_words_test() {
    assert_eq!(f_words(""), 0);
    assert_eq!(f_words(" "), 0);
    assert_eq!(f_words("a"), 1);
    assert_eq!(f_words("aaa"), 1);
    assert_eq!(f_words(" aaa   "), 1);
    assert_eq!(f_words(" aa   bbb"), 2);
    assert_eq!(f_words(" aa   bbb c  "), 3);
    assert_eq!(f_words("a a   bbb c ccc"), 5);
}

fn f_replace(s: &str, co: char, cn: char) -> String {
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
fn f_replace_test() {
    assert_eq!(f_replace("a", 'x', 'y'), "a");
    assert_eq!(f_replace("a", 'a', 'b'), "b");
    assert_eq!(f_replace("a b c", ' ', '_'), "a_b_c");
    assert_eq!(f_replace("abab", 'a', '\u{40}'), "@b@b");
}
