//
//
//

struct Point {
    x: i32,
    y: i32,
}

enum Command {
    Turn(u32),
    Move(u32),
    LightOn,
    LightOff,
}

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

    //

    let x = Some(5);
    let _y = 10;

    let z = match x {
        Some(50) => "500",
        Some(_y) => "y",
        _ => "none",
    };

    assert_eq!(z, "y");
    println!("match result: {}", z);

    //

    for x in 1..10 {
        match x {
            1 | 2 => println!("1st choice"),
            3...7 => println!("2nd choice"),
            _ => println!("3rd choice"),
        }
    }

    // destructuring structs I

    let p = Point { x: 0, y: 7 };
    let Point { x: a1, y: b1 } = p;
    let Point { x, y } = p;

    assert_eq!(0, a1);
    assert_eq!(7, b1);
    assert_eq!(0, x);
    assert_eq!(7, y);

    // destructuring structs II

    let mut polygon = vec![
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
    ];

    while let Some(point) = polygon.pop() {
        match point {
            Point { x: 0, y } => println!("first zero, second {}", y),
            Point { x, y: 0 } => println!("second zero, first {}", x),
            Point { x, y } => println!("3rd choice: {}/{}", x, y),
        }
    }

    // destructuring enums

    let mut batch = vec![
        Command::Turn(45),
        Command::Move(10),
        Command::LightOn,
        Command::Turn(90),
        Command::Move(3),
        Command::LightOff,
    ];

    while let Some(cmd) = batch.pop() {
        match cmd {
            Command::LightOn => println!("enable lights"),
            Command::LightOff => println!("disable lights"),
            Command::Turn(a) => println!("rotate: {} degrees", a),
            Command::Move(d) => println!("move: {} cm", d),
        }
    }

    // destructuring references

    let points = vec![
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
    ];

    let value: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("sum of squares: {}", value);

    // ignoring values in patterns

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (a, _, c, _, e) => println!("Some numbers: {}, {}, {}", a, c, e),
    }

    // ignoring remaining parts

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let seq = (1, 2, 3, 4, 5, 6);

    match seq {
        (fst, .., lst) => println!("first: {}, last: {}", fst, lst),
    }

    // creating references in patterns

    let cat_name = Some(String::from("Barsik"));

    match cat_name {
        Some(ref name) => println!("Cat: {}", name),
        None => println!("No cats..."),
    }

    // NB: 'cat_name' ownership was not taken by 'name' !
    println!("Cat: {:?}", cat_name);

    // creating mutable references in patterns

    let mut cat_name = Some(String::from("Barsik"));

    match cat_name {
        Some(ref mut name) => *name = String::from("Murka"),
        None => println!("No cats..."),
    }

    println!("Cat: {:?}", cat_name);

    // guards

    let mut seq = vec![1, 3, 5, 6, 7];

    while let Some(n) = seq.pop() {
        match n {
            1 => println!("1"),
            2 | 3 => println!("2 or 3"),
            n if n > 3 && n < 7 => println!("(3, 7)"),
            _ => println!("default: {}", n),
        }
    }

    // @ binding

    let mut seq = vec![1, 3, 5, 6, 7];

    while let Some(n) = seq.pop() {
        match n {
            1...3 => println!("some value in range [1, 3]"),
            val @ 4...6 => println!("known value in another range: {}", val),
            val @ _ => println!("known default value:{}", val),
        }
    }
}
