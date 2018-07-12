fn main() {
    //println!("run tests: cargo test --bin test-match");

    let s: Option<&str> = None;

    if let Some(s) = s {
        println!("s = {}", s);
    } else {
        println!("s = None");
    }

    //

    let s: Option<&str> = Some("hello");

    if let Some(s) = s {
        println!("s = {}", s);
    } else {
        println!("s = None");
    }

    //

    let n: Result<u8, _> = "10".parse();

    if let Ok(n) = n {
        println!("n = {}", n);
    } else {
        println!("parsing failed");
    }

    //

    let n: Result<u8, _> = "a".parse();

    if let Ok(n) = n {
        println!("n = {}", n);
    } else {
        println!("parsing failed");
    }

    //

    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
