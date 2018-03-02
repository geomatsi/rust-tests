fn main() {
    let s1 = String::from("abcd");
    let res;

    {
        // note that string literals always have 'static lifetime
        let s2 = "xyz000";
        res = longest(s1.as_str(), s2);
    }

    println!("The longest string is {}", res);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
