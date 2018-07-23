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

    //

    let p = Point { x: 0, y: 7 };
    let Point { x: a1, y: b1 } = p;
    let Point { x, y } = p;

    assert_eq!(0, a1);
    assert_eq!(7, b1);
    assert_eq!(0, x);
    assert_eq!(7, y);

    //

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

    //

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

    //
}
