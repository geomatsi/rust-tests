use std::clone;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    model: String,
}

impl clone::Clone for Shoe {
    fn clone(&self) -> Shoe {
        Shoe {
            size: self.size,
            model: self.model.clone(),
        }
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(m: u32) -> Counter {
        Counter { count: 0, max: m }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

impl clone::Clone for Counter {
    fn clone(&self) -> Counter {
        Counter {
            count: self.count,
            max: self.max,
        }
    }
}

//

fn shoes_in_my_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .cloned()
        .filter(|s| s.size <= shoe_size)
        .collect()
}

fn main() {
    // Example 1

    let shoes = vec![
        Shoe {
            size: 10,
            model: String::from("Asics"),
        },
        Shoe {
            size: 13,
            model: String::from("Mizuno"),
        },
        Shoe {
            size: 10,
            model: String::from("Hoka"),
        },
        Shoe {
            size: 11,
            model: String::from("Saucony"),
        },
    ];

    let shoes_list = shoes.iter();
    println!("ALL Shoes:");
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    let shoes_list = shoes_in_my_size(&shoes, 11);
    println!("My Shoes:");
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    let shoes_list = shoes.iter();
    println!("ALL Shoes:");
    for s in shoes_list {
        println!("  model: {}, size: {}", s.model, s.size);
    }

    println!("Shoe list {:?}", shoes);

    // Example 2

    let c = Counter::new(5);

    println!("c = {:?}", c);

    for s in c.clone() {
        println!("counter values: {}", s);
    }

    for s in c.clone().map(|x| x * x).filter(|x| (*x < 15)) {
        println!("counter filtered: {}", s);
    }

    println!("c = {:?}", c);

    for s in c {
        println!("counter: {}", s);
    }

    // no more can use c: it has been moved into iterator
}

// function that takes ownership
fn bogus_var_move<T>(names: Vec<T>)
where
    T: std::fmt::Debug,
{
    for s in names {
        println!("counter: {:?}", s);
    }
}

// use iter on array of &str
#[test]
fn f_test_iter_ref() {
    let names = vec!["a", "b", "c"];

    let bytes = names
        .iter()
        .map(|n: &&str| n.len())
        .fold(0, |acc, len| acc + len);

    assert_eq!(bytes, 3);

    bogus_var_move(names);

    // 'names' has been moved: can't be used
}

#[test]
fn f_test_iter_val() {
    let mut names: Vec<String> = Vec::new();

    let a = String::from("a");
    let b = String::from("b");
    let c = String::from("c");

    names.push(a);
    names.push(b);
    names.push(c);

    assert_eq!(names, vec!["a", "b", "c"]);

    let bytes = names
        .iter()
        .map(|n: &String| n.len())
        .fold(0, |acc, len| acc + len);

    assert_eq!(bytes, 3);

    bogus_var_move(names);

    // 'names' has been moved: can't be used
}
